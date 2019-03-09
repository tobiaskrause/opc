extern crate oaidl;
extern crate winapi;
extern crate winrt;
extern crate widestring;

use opc::backend::*;
use self::winapi::shared::wtypes::BSTR;
use self::oaidl::*;
use std::*;
use self::widestring::U16String;

pub struct ItemIterator{
    opc_browser: *mut ::gbdaaut::OPCBrowser,
    count: i32,
    pos: i32
}

impl ItemIterator {
    fn new(opc_browser: *mut ::gbdaaut::OPCBrowser) -> Result<ItemIterator> {
        let count = &mut 0i32;
        unsafe {
            let ref_opc_browser = &*opc_browser;
            let hr = ref_opc_browser.get_Count(count);
            if winapi::shared::winerror::SUCCEEDED(hr) {
                Ok(ItemIterator{opc_browser: opc_browser, count: *count, pos: 1})
            } else {
                Err(format!("get_count from opc_browser failed with err={}", hr))
            }
        }
    }

    fn browser(&self) -> &::gbdaaut::OPCBrowser {
        unsafe {
            &*self.opc_browser
        }
    }
}

impl Iterator for ItemIterator {
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

impl Drop for ItemIterator {
    fn drop(&mut self) {
        unsafe {
            self.browser().Release();
        }
    }
}

pub struct ComOPCBrowser {
    opc_browser: *mut ::gbdaaut::OPCBrowser
}

impl ComOPCBrowser {
    pub fn new(opc_browser: *mut ::gbdaaut::OPCBrowser) -> ComOPCBrowser {
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

impl OPCBrowser for ComOPCBrowser {
    fn into_iter(self) -> Box<Iterator<Item = Name>> {
        Box::new(ItemIterator::new(self.opc_browser).unwrap())
    }
}

#[cfg(test)]
mod test {
    // Todo: Add unit tests
}


