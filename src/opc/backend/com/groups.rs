extern crate oaidl;
extern crate winapi;
extern crate winrt;
extern crate widestring;
extern crate try_from;

use opc::backend::*;
use opc::backend::com::items::*;
use self::winapi::shared::wtypes::BSTR;
use self::oaidl::*;
use self::winapi::um::oaidl::*;
use std::*;
use std::ptr::NonNull;
use self::try_from::*;
use self::widestring::U16String;
use self::winrt::BStr;


pub struct ComOPCGroup {
    opc_group: *mut ::gbdaaut::IOPCGroup
}

impl ComOPCGroup {
    fn new(opc_group: *mut ::gbdaaut::IOPCGroup) -> ComOPCGroup {
        ComOPCGroup{opc_group}
    }

    fn group(&self) -> &::gbdaaut::IOPCGroup {
        unsafe{
            &*self.opc_group
        }
    }

    pub fn get_name(&self) -> Result<String> {
        unsafe {
            let mut id_bstr = BStr::empty();
            let hr = self.group().get_Name(&mut id_bstr.get());
            if winapi::shared::winerror::SUCCEEDED(hr) {
                Ok(id_bstr.to_string())
            } else {
                Err(format!("get_name from opc_group failed with err={:x}", hr))
            }
        }
    }

    pub fn get_items(&self) -> Result<ComOPCItems> {
        let mut opc_items_ptr: *mut ::gbdaaut::OPCItems = ::std::ptr::null_mut();
        unsafe {
            let hr = self.group().get_OPCItems((&mut opc_items_ptr) as *mut *mut ::gbdaaut::OPCItems);
            if !winapi::shared::winerror::SUCCEEDED(hr)
            {
                return Err(format!("get_items failed with err={:X}", hr ));
            }
            Ok(ComOPCItems::new(opc_items_ptr as *mut ::gbdaaut::OPCItems))
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
    opc_groups: *mut ::gbdaaut::IOPCGroups
}

impl ComOPCGroups {
    fn new(opc_groups: *mut ::gbdaaut::IOPCGroups) -> ComOPCGroups {
        ComOPCGroups{opc_groups}
    }
    fn groups(&self) -> &::gbdaaut::IOPCGroups {
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
            let mut opc_group_ptr: *mut ::gbdaaut::OPCGroup = ::std::ptr::null_mut();
            let hr = self.groups().Item(*group_variant, &mut opc_group_ptr);
            if winapi::shared::winerror::SUCCEEDED(hr) {
                Ok(ComOPCGroup::new(opc_group_ptr as *mut ::gbdaaut::IOPCGroup))
            } else {
                Err(format!("item from opc_items failed with err={:x}", hr))
            }
        }
    }

    pub fn find_group(&self, name: &str) -> Option<ComOPCGroup> {
        let mut iter = GroupIterator::new(self).unwrap();
        iter.find(|ref x| x.get_name().unwrap() == name)
    }

    pub fn add_group(&self, name: &str ) -> Result<ComOPCGroup> {
        unsafe {
            let default= VariantExt::<*mut u16>::into_variant(U16String::from_str(name)).unwrap().as_ptr();
            let mut opc_group_ptr: *mut ::gbdaaut::OPCGroup= ::std::ptr::null_mut();
            let hr = self.groups().Add(*default, &mut opc_group_ptr);
            if winapi::shared::winerror::SUCCEEDED(hr) {
                Ok(ComOPCGroup::new(opc_group_ptr as * mut ::gbdaaut::IOPCGroup))
            } else {
                Err(format!("add_group from opc_groups failed with err={:x}", hr))
            }
        }
    }

    pub fn remove_group(name: &str ) -> Result<()> {
        Ok(())
    }
}

impl TryFrom<&::gbdaaut::IOPCAutoServer> for ComOPCGroups {
    type Err = String;

    fn try_from(item: &::gbdaaut::IOPCAutoServer) -> result::Result<Self, Self::Err> {
        let mut opc_group_ptr: *mut ::gbdaaut::OPCGroups = ::std::ptr::null_mut();
        unsafe {
            let hr = item.get_OPCGroups((&mut opc_group_ptr) as *mut *mut ::gbdaaut::OPCGroups);
            if !winapi::shared::winerror::SUCCEEDED(hr)
            {
                return Err(format!("try_from failed with err={}", hr ));
            }
            Ok(ComOPCGroups::new(opc_group_ptr as *mut ::gbdaaut::IOPCGroups))
        }
    }
}

#[cfg(test)]
mod test {
    // Todo: Add unit tests
}