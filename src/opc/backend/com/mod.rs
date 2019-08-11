pub mod server;
mod browser;
mod groups;
mod items;

const SOURCE_DEVICE: i16 = 2;

#[cfg(test)]
pub mod test {
    #![allow(non_camel_case_types, non_snake_case, unused, non_upper_case_globals)]
    pub mod fake {
        use winapi::shared::winerror::HRESULT;
        use winapi::shared::wtypes::BSTR;
        use winapi::shared::guiddef::GUID;
        use winapi::um::oaidl::VARIANT;

        pub struct OPCBrowser;
        impl OPCBrowser {
            pub unsafe fn get_Count(&self, Count: *mut i32) -> HRESULT {
                0i32
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
                0i32
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

            pub fn Release(&self) {
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