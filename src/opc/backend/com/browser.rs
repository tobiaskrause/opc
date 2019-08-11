extern crate oaidl;
extern crate winapi;
extern crate winrt;
extern crate widestring;
extern crate try_from;

use opc::backend::*;
use self::winapi::shared::wtypes::BSTR;
use self::oaidl::*;
use std::*;
use self::try_from::*;
use self::widestring::U16String;
#[cfg(test)]
use super::test::fake as gbdaaut;
#[cfg(not(test))]
use ::gbdaaut;


pub struct ItemIdIterator {
    opc_browser: *mut gbdaaut::OPCBrowser,
    count: i32,
    pos: i32
}

impl ItemIdIterator {
    fn new(opc_browser: *mut gbdaaut::OPCBrowser) -> Result<ItemIdIterator> {
        let mut count = 0i32;
        unsafe {
            let ref_opc_browser = &*opc_browser;
            let hr = ref_opc_browser.get_Count(&mut count);
            if winapi::shared::winerror::SUCCEEDED(hr) {
                Ok(ItemIdIterator {opc_browser, count, pos: 1})
            } else {
                Err(format!("get_count from opc_browser failed with err={}", hr))
            }
        }
    }

    fn browser(&self) -> &gbdaaut::OPCBrowser {
        unsafe {
            &*self.opc_browser
        }
    }
}

impl Iterator for ItemIdIterator {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos < self.count {
            self.pos += 1;
            let name_bstr: *mut BSTR = winrt::BStr::empty().get_address();
            unsafe {
                let hr = self.browser().Item(
                    *(VariantExt::<i32>::into_variant(self.pos).unwrap().as_ptr()),
                    name_bstr
                );

                if winapi::shared::winerror::SUCCEEDED(hr) {
                    let name = U16String::from_bstr(*name_bstr).to_string_lossy();
                    Some(name)
                } else {
                    None
                }
            }
        } else {
            None
        }
    }
}

impl Drop for ItemIdIterator {
    fn drop(&mut self) {
        unsafe {
            self.browser().Release();
        }
    }
}

pub struct ComOPCBrowser {
    opc_browser: *mut gbdaaut::OPCBrowser
}

impl ComOPCBrowser {
    fn new(opc_browser: *mut gbdaaut::OPCBrowser) -> ComOPCBrowser {
        unsafe {
            let ref_opc_browser = &*opc_browser;
            ref_opc_browser.MoveToRoot();
            let filter_str: BSTR = *winrt::BStr::empty().get_address();
            ref_opc_browser.put_Filter(filter_str);
            ref_opc_browser.ShowLeafs(*(VariantExt::<i16>::into_variant(VariantBool::from(true)).unwrap().as_ptr()));
        }
        ComOPCBrowser{opc_browser}
    }
}

impl TryFrom<&gbdaaut::IOPCAutoServer> for ComOPCBrowser {
    type Err = String;

    fn try_from(item: &gbdaaut::IOPCAutoServer) -> result::Result<Self, Self::Err> {
        let mut opc_browser_ptr: *mut gbdaaut::OPCBrowser = ::std::ptr::null_mut();
        unsafe {
            let hr = item.CreateBrowser(&mut opc_browser_ptr);
            if !winapi::shared::winerror::SUCCEEDED(hr)
            {
                return Err(format!("CreateBrowser failed with err={}", hr ));
            }
        }
        Ok(ComOPCBrowser::new(opc_browser_ptr))
    }
}

impl OPCBrowser for ComOPCBrowser {
    fn into_iter(self) -> Box<Iterator<Item = Name>> {
        Box::new(ItemIdIterator::new(self.opc_browser).unwrap())
    }
}

#[cfg(test)]
mod test {

    use super::ItemIdIterator;
    #[test]
    fn test() {
        let mut fake_browser = super::super::test::fake::OPCBrowser{};
        ItemIdIterator::new(&mut fake_browser).unwrap();
    }
}