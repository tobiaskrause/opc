pub mod server;
mod browser;
mod groups;
mod items;

const SOURCE_DEVICE: i16 = 2;

#[cfg(test)]
pub mod test {
    #![allow(non_camel_case_types, non_snake_case, unused, non_upper_case_globals)]
    pub mod fake {
        extern crate oaidl;
        use winapi::shared::winerror::HRESULT;
        use winapi::shared::wtypes::BSTR;
        use winapi::shared::guiddef::GUID;
        use winapi::um::oaidl::*;
        use self::oaidl::*;


        use queues::*;

        use std::cell::*;

        const NOT_FOUND: HRESULT = -1;

        #[derive(Clone,Copy)]
        pub enum OPCBrowserCalls {
            None,
            get_count{ exp_Count: *mut i32, result: HRESULT},
            ShowLeafs{exp_Flat: VARIANT, result: HRESULT},
            put_Filter{exp_Filter: BSTR, result: HRESULT},
            MoveToRoot{result: HRESULT},
            Item{exp_ItemSpecifier: VARIANT, exp_Item: *mut BSTR, result: HRESULT},
            Release{result: HRESULT}
        }

        pub struct OPCBrowser {
            exps: RefCell<Queue<OPCBrowserCalls>>
        }

        impl OPCBrowser {

            pub fn new(exps : RefCell<Queue<OPCBrowserCalls>>) -> OPCBrowser {
                OPCBrowser{exps}
            }

            fn next_exp(&self) -> Option<OPCBrowserCalls>  {
                self.exps.borrow_mut().remove().map(|call| Option::Some(call)).unwrap_or(Option::None)
            }
            pub unsafe fn get_Count(&self, Count: *mut i32) -> HRESULT {
                if let Some(OPCBrowserCalls::get_count{exp_Count, result}) = self.next_exp() {
                    *Count = *exp_Count;
                    return result
                }
                panic!()
            }

            pub unsafe fn ShowLeafs(&self, Flat: VARIANT) -> HRESULT {
                0i32
            }

            pub unsafe fn put_Filter(&self, Filter: BSTR) -> HRESULT {
                0i32
            }

            pub unsafe fn MoveToRoot(&self) -> HRESULT {
                0i32
            }

            pub unsafe fn Item(&self, ItemSpecifier: VARIANT, Item: *mut BSTR) -> HRESULT {
                if let Some(OPCBrowserCalls::Item{exp_ItemSpecifier, exp_Item, result}) = self.next_exp() {
                    let mut copiedItemSpecifier = ItemSpecifier;
                    let mut copiedexp_ItemSpecifier = exp_ItemSpecifier;
                    let a: i32 = VariantExt::<i32>::from_variant(Ptr::with_checked(&mut copiedItemSpecifier).unwrap()).unwrap();
                    let b: i32 = VariantExt::<i32>::from_variant(Ptr::with_checked(&mut copiedexp_ItemSpecifier).unwrap()).unwrap();
                    if a == b {
                        *Item = *exp_Item;
                        return result
                    } else {
                        return -1
                    }
                }
                panic!()
            }

            pub fn Release(&self) {
            }
        }

        pub struct IOPCAutoServer;
        impl IOPCAutoServer {
            pub unsafe fn CreateBrowser(&self, ppBrowser: *mut *mut OPCBrowser) -> HRESULT {
                0i32
            }

            pub unsafe fn get_OPCGroups(&self, ppGroups: *mut *mut OPCGroups) -> HRESULT {
                0i32
            }

            pub unsafe fn Connect(&self, ProgID: BSTR, Node: VARIANT) -> HRESULT {
                0i32
            }

            pub unsafe fn Disconnect(&self) -> HRESULT {
                0i32
            }

            pub unsafe fn Release(&self) {
            }
        }

        impl winapi::Interface for IOPCAutoServer {
            fn uuidof() -> GUID {
                unimplemented!()
            }
        }

        pub struct OPCGroups;

        pub struct IOPCGroups;
        impl IOPCGroups{
            pub unsafe fn get_Count(&self, Count: *mut i32) -> HRESULT {
                0i32
            }

            pub unsafe fn Item(&self, ItemSpecifier: VARIANT, ppGroup: *mut *mut OPCGroup) -> HRESULT {
                0i32
            }

            pub unsafe fn Add(&self, Name: VARIANT, ppGroup: *mut *mut OPCGroup) -> HRESULT {
                0i32
            }
        }

        pub struct OPCGroup;
        pub struct IOPCGroup;
        impl IOPCGroup {
            pub unsafe fn get_Name(&self, Name: *mut BSTR) -> HRESULT {
                0i32
            }

            pub unsafe fn get_OPCItems(&self, ppItems: *mut *mut OPCItems) -> HRESULT {
                0i32
            }
        }

        pub struct OPCItems;
        impl OPCItems {
            pub unsafe fn AddItem(
                &self,
                ItemID: BSTR,
                ClientHandle: i32,
                ppItem: *mut *mut OPCItem,
            ) -> HRESULT {
                0i32
            }

            pub unsafe fn Item(&self, ItemSpecifier: VARIANT, ppItem: *mut *mut OPCItem) -> HRESULT {
                0i32
            }

            pub unsafe fn get_Count(&self, Count: *mut i32) -> HRESULT {
                0i32
            }
        }

        pub struct OPCItem;
        impl OPCItem {
            pub unsafe fn Write(&self, Value: VARIANT) -> HRESULT {
                0i32
            }

            pub unsafe fn Read(
                &self,
                Source: i16,
                Value: *mut VARIANT,
                Quality: *mut VARIANT,
                TimeStamp: *mut VARIANT,
            ) -> HRESULT {
                0i32
            }

            pub unsafe fn get_ItemID(&self, ItemID: *mut BSTR) -> HRESULT {
                0i32
            }
        }

        pub struct OPCServer;
        impl OPCServer {
            pub fn uuidof() -> GUID {
                unimplemented!()
            }
        }

    }
    // Todo: Add unit tests
}