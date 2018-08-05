extern crate winapi;
extern crate winrt;

use ::backend::*;
use winapi::um::oaidl::*;
use winapi::shared::wtypes::BSTR;

pub struct ComOPCServer<'a> {
    opc_wrapper: &'a ::gbdaaut::IOPCAutoServer
}

impl <'a> ComOPCServer<'a> {
    pub fn new() -> ComOPCServer<'a> {
        unsafe {
            ComOPCServer{opc_wrapper: &*(::std::ptr::null_mut() as *mut ::gbdaaut::IOPCAutoServer)}
        }
    }
}

impl <'a> OPCAutoServer for ComOPCServer<'a> {
    fn init(&mut self) {
        unsafe {
            let hr = winapi::um::objbase::CoInitialize(::std::ptr::null_mut());
            assert!(winapi::shared::winerror::SUCCEEDED(hr));

            let mut opc_wrapper: *mut winapi::ctypes::c_void = ::std::ptr::null_mut();
            let hr =
                winapi::um::combaseapi::CoCreateInstance(
                    &::gbdaaut::OPCServer::uuidof(),
                    ::std::ptr::null_mut(),
                    winapi::um::combaseapi::CLSCTX_ALL,
                    &<::gbdaaut::IOPCAutoServer as winapi::Interface>::uuidof(),
                    &mut opc_wrapper
                );
            println!("Result {}",hr);
            assert!(winapi::shared::winerror::SUCCEEDED(hr));
            self.opc_wrapper = &*(opc_wrapper as *mut ::gbdaaut::IOPCAutoServer);
        }
    }
    fn connect(&self, server_name: &str) {
        unsafe {
            let server: BSTR = *winrt::BStr::from(server_name).get_address();
            let node: VARIANT = ::std::mem::zeroed();
            let hr = self.opc_wrapper.Connect(server, node);
            println!("Result {}",hr);
            assert!(winapi::shared::winerror::SUCCEEDED(hr));
        } 
    }

    fn read_value(&self, variable_name: &str) -> String {
        String::from(format!("value of {}", variable_name))
    }
    
    fn disconnect(&self) {
        unsafe {
            let hr = self.opc_wrapper.Disconnect();
            assert!(winapi::shared::winerror::SUCCEEDED(hr));
        }
    }
}

impl <'a> Drop for ComOPCServer<'a> {
    fn drop(&mut self) {
        self.disconnect();
        unsafe {
            self.opc_wrapper.Release();
        }
    }
}

#[cfg(test)]
mod test {
    use super::ComOPCServer;
    use opc::backend::OPCAutoServer;

    const SERVICE_NAME: &str = "Graybox.Simulator.1";

    fn get_instance<'a>() -> ComOPCServer<'a> {
        let mut instance = ComOPCServer::new();
        instance.init();
        instance
    }

    fn connect_with_simulator<'a>() -> ComOPCServer<'a> {
        let instance = get_instance();
        instance.connect(SERVICE_NAME);
        instance
    }

    #[test]
    fn connect_disconnect_test() {
        let instance = connect_with_simulator();
        instance.disconnect();
    }

    #[test]
    fn connect_drop_test() {
        {
            let instance = connect_with_simulator();
        }
    }

    #[test]
    fn read_success_test() {
        let instance = connect_with_simulator();
        let value = instance.read_value("test1");
        instance.disconnect();
    }

    #[test]
    fn read_error_test() {
        let instance = connect_with_simulator();
        let value = instance.read_value("test1");
        instance.disconnect();
    }
}