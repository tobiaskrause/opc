#![allow(unstable_name_collisions)]
extern crate oaidl;
extern crate winapi;
extern crate winrt;
extern crate widestring;
extern crate try_from;

use opc::backend::*;
use opc::backend::com::browser::*;
use opc::backend::com::groups::*;
use opc::backend::com::items::*;
use self::winapi::um::oaidl::*;
use self::winapi::shared::wtypes::BSTR;
use self::try_from::TryFrom;
use std::*;

#[cfg(test)]
use super::test::fake as gbdaaut;
#[cfg(not(test))]
use ::gbdaaut;

pub struct ComOPCServer<'a> {
    opc_wrapper: &'a gbdaaut::IOPCAutoServer
}

impl <'a> ComOPCServer<'a> {
    pub fn try_new() -> Result<ComOPCServer<'a>> {
        unsafe {
            let mut this = ComOPCServer{opc_wrapper: &*(::std::ptr::null_mut() as *mut gbdaaut::IOPCAutoServer)};
            this.init().map(|_v| this)
        }
    }

    fn init(&mut self) -> Result<()> {
        unsafe {
            let hr = winapi::um::objbase::CoInitialize(::std::ptr::null_mut());
            if !winapi::shared::winerror::SUCCEEDED(hr)
            {
                return Err(format!("CoInitialize failed with err={}", hr ));
            }
            let mut opc_wrapper: *mut winapi::ctypes::c_void = ::std::ptr::null_mut();
            let hr2 =
                winapi::um::combaseapi::CoCreateInstance(
                    &gbdaaut::OPCServer::uuidof(),
                    ::std::ptr::null_mut(),
                    winapi::um::combaseapi::CLSCTX_ALL,
                    &<gbdaaut::IOPCAutoServer as winapi::Interface>::uuidof(),
                    &mut opc_wrapper
                );
            if !winapi::shared::winerror::SUCCEEDED(hr2)
            {
                return Err(format!("CoCreateInstance failed with err={}", hr2 ));
            }
            self.opc_wrapper = &*(opc_wrapper as *mut gbdaaut::IOPCAutoServer);
            Ok(())
        }
    }

    fn get_group(&self, group_name: &str) -> Result<ComOPCGroup> {
        let groups = ComOPCGroups::try_from(self.opc_wrapper)?;
        match groups.find_group(group_name) {
            None => return groups.add_group(group_name),
            Some(group) => return Ok(group)
        }
    }

    fn get_item(&self, group: &ComOPCGroup, variable_name: &str) -> Result<ComOPCItem> {
        let items = group.get_items().unwrap();
        match items.find_item(variable_name) {
            None => return items.add_item(variable_name),
            Some(item) => return Ok(item)
        }
    }
}

impl <'a> OPCAutoServer for ComOPCServer<'a> {
    fn connect(&self, server_name: &str) -> Result<()>{
        unsafe {
            let server: BSTR = *winrt::BStr::from(server_name).get_address();
            let node: VARIANT = ::std::mem::zeroed();
            let hr = self.opc_wrapper.Connect(server, node);
            if !winapi::shared::winerror::SUCCEEDED(hr)
            {
                return Err(format!("CoCreateInstance failed with err={}", hr ));
            }
        }
        Ok(())
    }

    fn read_value(&self, variable_name: &str) -> Result<String> {
        let group = self.get_group("default_reader")?;
        let item = self.get_item(&group, variable_name)?;
        item.read()
    }

    fn list_names(&self) -> Result<Vec<Name>> {
        let browser = ComOPCBrowser::try_from(self.opc_wrapper)?;
        Ok(browser.into_iter().collect())
    }

    fn write_value(&self, variable_name: &str, value: &str) -> Result<()> {
        let group = self.get_group("default_writer")?;
        let item = self.get_item(&group, variable_name)?;
        item.write(value)
    }

    fn disconnect(&self) -> Result<()> {
        unsafe {
            let hr = self.opc_wrapper.Disconnect();
            if !winapi::shared::winerror::SUCCEEDED(hr)
            {
                return Err(format!("CoCreateInstance failed with err={}", hr ));
            }
        }
        Ok(())
    }
}

impl <'a> Drop for ComOPCServer<'a> {
    fn drop(&mut self) {
        self.disconnect().unwrap();
        unsafe {
            self.opc_wrapper.Release();
        }
    }
}