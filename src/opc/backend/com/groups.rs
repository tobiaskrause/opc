extern crate oaidl;
extern crate winapi;
extern crate winrt;
extern crate widestring;
extern crate try_from;

use opc::backend::*;
use opc::backend::com::items::*;
use self::oaidl::*;
use std::*;
use self::try_from::*;
use self::widestring::U16String;
use self::winapi::shared::wtypes::BSTR;

#[cfg(test)]
use super::test::fake as gbdaaut;
#[cfg(not(test))]
use gbdaaut;

pub struct ComOPCGroup {
    opc_group: *mut gbdaaut::IOPCGroup
}

impl ComOPCGroup {
    fn new(opc_group: *mut gbdaaut::IOPCGroup) -> ComOPCGroup {
        ComOPCGroup{opc_group}
    }

    fn group(&self) -> &gbdaaut::IOPCGroup {
        unsafe{
            &*self.opc_group
        }
    }

    pub fn get_name(&self) -> Result<String> {
        unsafe {
            let id_bstr: *mut BSTR = winrt::BStr::empty().get_address();
            let hr = self.group().get_Name(id_bstr);
            if winapi::shared::winerror::SUCCEEDED(hr) {
                Ok(U16String::from_bstr(*id_bstr).to_string_lossy())
            } else {
                Err(format!("get_name from opc_group failed with err={:x}", hr))
            }
        }
    }

    pub fn get_items(&self) -> Result<ComOPCItems> {
        let mut opc_items_ptr: *mut gbdaaut::OPCItems = ::std::ptr::null_mut();
        unsafe {
            let hr = self.group().get_OPCItems((&mut opc_items_ptr) as *mut *mut gbdaaut::OPCItems);
            if !winapi::shared::winerror::SUCCEEDED(hr)
            {
                return Err(format!("get_items failed with err={:X}", hr ));
            }
            Ok(ComOPCItems::new(opc_items_ptr as *mut gbdaaut::OPCItems))
        }
    }
}

pub struct GroupIterator <'a>{
    opc_groups: &'a ComOPCGroups,
    count: i32,
    pos: i32
}

impl <'a> GroupIterator <'a> {
    fn new(opc_groups: &'a ComOPCGroups) -> Result<GroupIterator> {
        let count = opc_groups.count()?;
        Ok(GroupIterator {opc_groups, count , pos: 1})
    }
}

impl <'a> Iterator for GroupIterator <'a> {
    type Item = ComOPCGroup;
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos < self.count {
            self.pos += 1;
            if let Ok(item) = self.opc_groups.item(self.pos.clone()) {
                return Some(item);
            } else {
                return None;
            }
        } else {
            None
        }
    }
}

pub struct ComOPCGroups {
    opc_groups: *mut gbdaaut::IOPCGroups
}

impl ComOPCGroups {
    fn new(opc_groups: *mut gbdaaut::IOPCGroups) -> ComOPCGroups {
        ComOPCGroups{opc_groups}
    }
    fn groups(&self) -> &gbdaaut::IOPCGroups {
        unsafe {
            &*self.opc_groups
        }
    }

    pub fn count(&self) -> Result<i32>{
        unsafe {
            let mut count = 0i32;
            let hr = self.groups().get_Count(&mut count);
            if winapi::shared::winerror::SUCCEEDED(hr) {
                Ok(count)
            } else {
                Err(format!("get_count from opc_groups failed with err={}", hr))
            }
        }
    }

    pub fn item(&self, group_id: i32) -> Result<ComOPCGroup> {
        unsafe {
            let group_variant = VariantExt::<i32>::into_variant(group_id).unwrap().as_ptr();
            let mut opc_group_ptr: *mut gbdaaut::OPCGroup = ::std::ptr::null_mut();
            let hr = self.groups().Item(*group_variant, &mut opc_group_ptr);
            if winapi::shared::winerror::SUCCEEDED(hr) {
                Ok(ComOPCGroup::new(opc_group_ptr as *mut gbdaaut::IOPCGroup))
            } else {
                Err(format!("item from opc_items failed with err={:x}", hr))
            }
        }
    }

    pub fn find_group(&self, name: &str) -> Option<ComOPCGroup> {
        let mut iter = GroupIterator::new(self).unwrap();
        iter.find(|ref x| x.get_name().unwrap() == String::from(name))
    }

    pub fn add_group(&self, name: &str ) -> Result<ComOPCGroup> {
        unsafe {
            let default= VariantExt::<*mut u16>::into_variant(U16String::from_str(name)).unwrap().as_ptr();
            let mut opc_group: *mut gbdaaut::OPCGroup = ::std::ptr::null_mut();
            let opc_group_ptr = &mut opc_group as *mut *mut gbdaaut::OPCGroup;
            let hr = self.groups().Add(*default, opc_group_ptr);
            if winapi::shared::winerror::SUCCEEDED(hr) {
                Ok(ComOPCGroup::new(opc_group as * mut gbdaaut::IOPCGroup))
            } else {
                Err(format!("add_group from opc_groups failed with err={:x}", hr))
            }
        }
    }
}

impl TryFrom<&gbdaaut::IOPCAutoServer> for ComOPCGroups {
    type Err = String;

    fn try_from(item: &gbdaaut::IOPCAutoServer) -> result::Result<Self, Self::Err> {
        let mut opc_group_ptr: *mut gbdaaut::OPCGroups = ::std::ptr::null_mut();
        unsafe {
            let hr = item.get_OPCGroups((&mut opc_group_ptr) as *mut *mut gbdaaut::OPCGroups);
            if !winapi::shared::winerror::SUCCEEDED(hr)
            {
                return Err(format!("try_from failed with err={}", hr ));
            }
            Ok(ComOPCGroups::new(opc_group_ptr as *mut gbdaaut::IOPCGroups))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use super::super::test::*;

    use queue::*;
    use std::cell::*;

    const HRESULT_OK: i32 = 0;
    const HRESULT_FAIL: i32 = -1;

    #[test]
    fn test_com_opc_groups() {
        use super::super::test::fake::IOPCGroupsCalls::*;
        unsafe {
            let mut exp_count: i32 = 1;
            let exp_group_1 = fake::OPCGroup{ exps: RefCell::new(Queue::<fake::IOPCGroupCalls>::new())};
            let exp_group_2 = fake::OPCGroup{ exps: RefCell::new(Queue::<fake::IOPCGroupCalls>::new())};

            let item_spec_1 = *(VariantExt::<i32>::into_variant(1i32).unwrap().as_ptr());
            let item_spec_2 = *(VariantExt::<i32>::into_variant(2i32).unwrap().as_ptr());
            let exps= utils::expectations::<fake::IOPCGroupsCalls>(&[
                get_count { exp_Count: &mut exp_count, result: HRESULT_OK },
                Item { exp_ItemSpecifier: item_spec_1, exp_ppGroup: Box::new(exp_group_1), result: HRESULT_OK },
                Item { exp_ItemSpecifier: item_spec_2, exp_ppGroup: Box::new(exp_group_2), result: HRESULT_FAIL }
            ]);
            let com_opc_groups = ComOPCGroups::new(&mut fake::IOPCGroups::new(exps));
            assert_eq!(com_opc_groups.count(), Ok(exp_count));
            assert_eq!(com_opc_groups.item(1).is_ok(), true);
            assert_eq!(com_opc_groups.item(exp_count + 1).is_err(), true);
        }
    }

    #[test]
    fn test_com_opc_group() {
        use super::super::test::fake::IOPCGroupCalls::*;
        use super::winrt::BStr;

        let mut exp_items: *mut fake::OPCItems  = &mut fake::OPCItems{};
        let filter_str: BStr = winrt::BStr::from("abc");
        let exps= utils::expectations::<fake::IOPCGroupCalls>(&[
            get_Name { exp_Name: &mut filter_str.get(), result: HRESULT_OK },
            get_OPCItems { exp_ppItems: &mut exp_items, result: HRESULT_OK }
        ]);
        let com_opc_group = ComOPCGroup::new(&mut fake::IOPCGroup::new(exps));
        assert_eq!(com_opc_group.get_name(), Ok(String::from("abc")));
        assert_eq!(com_opc_group.get_items().is_ok(), true);
    }

}