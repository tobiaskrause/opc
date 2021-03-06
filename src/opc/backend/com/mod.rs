pub mod server;
mod browser;
mod groups;
mod items;

const SOURCE_DEVICE: i16 = 2;

#[cfg(test)]
pub mod test {
    #![allow(non_camel_case_types, non_snake_case, unused, non_upper_case_globals)]

    use opc::backend::com::test::fake::OPCGroup;

    pub mod utils {
        use queue::*;
        use std::cell::*;

        // TODO: rewrite as macro
        pub fn expectations<T: std::clone::Clone>(exps: &[T]) -> RefCell<Queue<T>> {
            let mut queue = Queue::<T>::new();
            for exp in exps {
                queue.queue(exp.clone()).unwrap();
            }
            RefCell::new(queue)
        }
    }
    pub mod fake {
        extern crate oaidl;
        use winapi::shared::winerror::HRESULT;
        use winapi::shared::wtypes::BSTR;
        use winapi::shared::guiddef::GUID;
        use winapi::um::oaidl::*;
        use self::oaidl::*;

        use queue::*;
        use std::cell::*;
        use std::boxed::Box;
        use winapi::_core::fmt::{Formatter, Error};
        use winapi::_core::intrinsics::write_bytes;

        const NOT_FOUND: HRESULT = -1;

        #[derive(Clone)]
        pub enum  OPCBrowserCalls {
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
                self.exps.borrow_mut().dequeue().map(|call| Option::Some(call)).unwrap_or(Option::None)
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

            pub unsafe fn Release(&self) {
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

        #[derive(Clone)]
        pub enum IOPCGroupsCalls {
            None,
            get_count{ exp_Count: *mut i32, result: HRESULT},
            Item{exp_ItemSpecifier: VARIANT, exp_ppGroup: Box<OPCGroup>, result: HRESULT},
            Add{exp_ItemSpecifier: VARIANT, exp_ppGroup: Box<OPCGroup>, result: HRESULT}
        }

        impl std::fmt::Debug for IOPCGroupsCalls {
            fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
                match *self {
                    IOPCGroupsCalls::None => write!(f, "None"),
                    IOPCGroupsCalls::get_count{exp_Count: _, result: _} => write!(f, "get_count"),
                    IOPCGroupsCalls::Item{exp_ItemSpecifier: _, exp_ppGroup: _, result: _} => write!(f, "Item"),
                    IOPCGroupsCalls::Add{exp_ItemSpecifier: _, exp_ppGroup: _, result: _} => write!(f, "Add")
                }
            }
        }

        pub struct OPCGroups{
            exps: RefCell<Queue<IOPCGroupsCalls>>
        }

        impl From<IOPCGroups> for OPCGroups {
            fn from(other: IOPCGroups) -> Self {
                OPCGroups{exps: other.exps}
            }
        }

        pub struct IOPCGroups {
            exps: RefCell<Queue<IOPCGroupsCalls>>
        }

        impl IOPCGroups {

            pub fn new(exps : RefCell<Queue<IOPCGroupsCalls>>) -> IOPCGroups { IOPCGroups{exps} }

            fn next_exp(&self) -> Option<IOPCGroupsCalls>  {
                self.exps.borrow_mut().dequeue().map(|call| Option::Some(call)).unwrap_or(Option::None)
            }

            pub unsafe fn get_Count(&self, Count: *mut i32) -> HRESULT {
                let exp = self.next_exp();
                if let Some(IOPCGroupsCalls::get_count{exp_Count, result}) = exp {
                    *Count = *exp_Count;
                    return result
                } else {
                    println!("Exp was {:?}", exp);
                    panic ! ()
                }

            }

            pub unsafe fn Item(&self, ItemSpecifier: VARIANT, ppGroup: *mut *mut OPCGroup) -> HRESULT {
                if let Some(IOPCGroupsCalls::Item{exp_ItemSpecifier, mut exp_ppGroup, result}) = self.next_exp() {
                    let mut copiedItemSpecifier = ItemSpecifier;
                    let mut copiedexp_ItemSpecifier = exp_ItemSpecifier;
                    let a: i32 = VariantExt::<i32>::from_variant(Ptr::with_checked(&mut copiedItemSpecifier).unwrap()).unwrap();
                    let b: i32 = VariantExt::<i32>::from_variant(Ptr::with_checked(&mut copiedexp_ItemSpecifier).unwrap()).unwrap();
                    if a == b {
                        *ppGroup = &mut *exp_ppGroup;
                        return result
                    } else {
                        return -1
                    }
                }
                panic!()
            }

            pub unsafe fn Add(&self, Name: VARIANT, ppGroup: *mut *mut OPCGroup) -> HRESULT {
                if let Some(IOPCGroupsCalls::Add{exp_ItemSpecifier, mut exp_ppGroup, result}) = self.next_exp() {
                    let mut copiedItemSpecifier = Name;
                    let mut copiedexp_ItemSpecifier = exp_ItemSpecifier;
                    let a: i32 = VariantExt::<i32>::from_variant(Ptr::with_checked(&mut copiedItemSpecifier).unwrap()).unwrap();
                    let b: i32 = VariantExt::<i32>::from_variant(Ptr::with_checked(&mut copiedexp_ItemSpecifier).unwrap()).unwrap();
                    if a == b {
                        *ppGroup = &mut *exp_ppGroup;
                        return result
                    } else {
                        return -1
                    }
                }
                panic!()
            }
        }

        #[derive(Clone,Copy)]
        pub enum IOPCGroupCalls {
            None,
            get_Name{exp_Name: *mut BSTR, result: HRESULT},
            get_OPCItems{exp_ppItems: *mut *mut OPCItems, result: HRESULT},
        }

        #[derive(Clone)]
        pub struct OPCGroup {
            pub exps: RefCell<Queue<IOPCGroupCalls>>
        }

        impl From<IOPCGroup> for OPCGroup {
            fn from(other: IOPCGroup) -> Self {
                OPCGroup{exps: other.exps}
            }
        }
        pub struct IOPCGroup {
            exps: RefCell<Queue<IOPCGroupCalls>>
        }

        impl IOPCGroup {

            pub fn new(exps : RefCell<Queue<IOPCGroupCalls>>) -> IOPCGroup {
                IOPCGroup{exps}
            }

            fn next_exp(&self) -> Option<IOPCGroupCalls>  {
                self.exps.borrow_mut().dequeue().map(|call| Option::Some(call)).unwrap_or(Option::None)
            }

            pub unsafe fn get_Name(&self, Name: *mut BSTR) -> HRESULT {
                if let Some(IOPCGroupCalls::get_Name{exp_Name, result}) = self.next_exp() {
                    *Name = *exp_Name;
                    return result
                }
                panic ! ()
            }

            pub unsafe fn get_OPCItems(&self, ppItems: *mut *mut OPCItems) -> HRESULT {
                if let Some(IOPCGroupCalls::get_OPCItems{exp_ppItems, result}) = self.next_exp() {
                    *ppItems = *exp_ppItems;
                    return result
                }
                panic ! ()
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