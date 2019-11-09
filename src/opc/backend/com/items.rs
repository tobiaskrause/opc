extern crate oaidl;
extern crate winapi;
extern crate winrt;
extern crate widestring;
extern crate try_from;

use opc::backend::*;
use opc::backend::com::*;
use self::oaidl::*;
use self::winapi::um::oaidl::*;
use std::*;
use std::ptr::NonNull;
use self::widestring::U16String;
use self::winrt::BStr;
#[cfg(test)]
use super::test::fake as gbdaaut;
#[cfg(not(test))]
use gbdaaut;


pub struct ItemIterator <'a>{
    opc_items: &'a ComOPCItems,
    count: i32,
    pos: i32
}

impl <'a> ItemIterator <'a> {
    fn new(opc_items: &'a ComOPCItems) -> Result<ItemIterator> {
        let count = opc_items.count()?;
        Ok(ItemIterator {opc_items, count , pos: 1})
    }
}

impl <'a> Iterator for ItemIterator <'a> {
    type Item = ComOPCItem;
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos < self.count {
            self.pos += 1;
            if let Ok(item) = self.opc_items.item(self.pos.clone()) {
                return Some(item);
            } else {
                return None;
            }
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct ComOPCItems {
    opc_items: *mut gbdaaut::OPCItems
}

impl ComOPCItems {
    pub fn new(opc_items: *mut gbdaaut::OPCItems) -> ComOPCItems {
        ComOPCItems{opc_items}
    }

    fn items(&self) -> &gbdaaut::OPCItems {
        unsafe {
            &*self.opc_items
        }
    }

    pub fn count(&self) -> Result<i32>{
        unsafe {
            let mut count =  0i32;
            let hr = self.items().get_Count(&mut count);
            if winapi::shared::winerror::SUCCEEDED(hr) {
                Ok(count)
            } else {
                Err(format!("get_count from opc_groups failed with err={}", hr))
            }
        }
    }

    pub fn item(&self, item_id: i32) -> Result<ComOPCItem> {
        unsafe {
            let item_variant = VariantExt::<i32>::into_variant(item_id).unwrap().as_ptr();
            let mut opc_item_ptr: *mut gbdaaut::OPCItem = ::std::ptr::null_mut();
            let hr = self.items().Item(*item_variant, &mut opc_item_ptr);
            if winapi::shared::winerror::SUCCEEDED(hr) {
                Ok(ComOPCItem::new(opc_item_ptr as *mut gbdaaut::OPCItem))
            } else {
                Err(format!("find_item from opc_items failed with err={:x}", hr))
            }
        }
    }

    pub fn find_item(&self, item_name: &str) -> Option<ComOPCItem> {
        let mut iter = ItemIterator::new(self).unwrap();
        iter.find(|ref x| x.get_item_id().unwrap() == item_name)
    }

    pub fn add_item(&self, item_name: &str) -> Result<ComOPCItem> {
        unsafe {
            let id_bstr = BStr::from(item_name);
            let mut opc_item_ptr: *mut gbdaaut::OPCItem = ::std::ptr::null_mut();
            let hr = self.items().AddItem(id_bstr.get(), 0, &mut opc_item_ptr);
            if winapi::shared::winerror::SUCCEEDED(hr) {
                Ok(ComOPCItem::new(opc_item_ptr as *mut gbdaaut::OPCItem))
            } else {
                Err(format!("add_item from opc_items failed with err={:x}", hr))
            }
        }
    }
}

pub struct ComOPCItem {
    opc_item: *mut gbdaaut::OPCItem
}

impl ComOPCItem {
    fn new(opc_item: *mut gbdaaut::OPCItem) -> ComOPCItem {
        ComOPCItem{opc_item}
    }

    fn item(&self) -> &gbdaaut::OPCItem {
        unsafe {
            &*self.opc_item
        }
    }

    pub fn get_item_id(&self) -> Result<String> {
        unsafe {
            let id_bstr = BStr::empty();
            let hr = self.item().get_ItemID(&mut id_bstr.get());
            if winapi::shared::winerror::SUCCEEDED(hr) {
                Ok(id_bstr.to_string())
            } else {
                Err(format!("get_item_id from opc_item failed with err={:x}", hr))
            }
        }
    }

    pub fn read(&self) -> Result<String> {
        unsafe {
            let mut value: VARIANT = ::std::mem::zeroed();
            let mut quality: VARIANT = ::std::mem::zeroed();
            let mut timestamp: VARIANT = ::std::mem::zeroed();
            let hr = self.item().Read(SOURCE_DEVICE, &mut value, &mut quality, &mut timestamp);
            if winapi::shared::winerror::SUCCEEDED(hr) {
                let value_ptr = Ptr::new(NonNull::new_unchecked(&mut value));
                let value_ustr : U16String = VariantExt::<_>::from_variant(value_ptr).unwrap();
                Ok(value_ustr.to_string_lossy())
            } else {
                Err(format!("read from opc_item failed with err={:x}", hr))
            }
        }
    }

    pub fn write(&self, value: &str) -> Result<()> {
        unsafe {
            let value = VariantExt::<_>::into_variant(U16String::from_str(value)).unwrap().as_ptr();
            let hr = self.item().Write(*value);
            if winapi::shared::winerror::SUCCEEDED(hr) {
                Ok(())
            } else {
                Err(format!("write from opc_item failed with err={:x}", hr))
            }
        }
    }
}

#[cfg(test)]
mod test {
    // Todo: Add unit tests
}