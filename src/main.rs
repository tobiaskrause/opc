#[macro_use]
extern crate winapi;
extern crate winrt;

pub mod gbdaaut;
pub mod opc;

use opc::*;
use opc::backend::*;

struct MockOPCBackend;

impl OPCAutoServer for MockOPCBackend {
    fn init(&mut self) -> Result<()>{
        Ok(())
    }

    fn connect(&self, server_name: &str) -> Result<()> {
        Ok(())
    }

    fn read_value(&self, variable_name: &str) -> Result<String> {
        Ok(String::from(format!("value of {}", variable_name)))
    }

    fn write_value(&self, variable_name: &str, value: &str) -> Result<()> {
        Ok(())
    }
    
    fn disconnect(&self) -> Result<()>{
        Ok(())
    }
}

fn main() {
    let opc_server = OPCServer::new();
    //let connection = opc_server.open("Freelance2000OPCServer.202.1");
    let connection = opc_server.open("Graybox.Simulator.1");
    println!("{}", connection.read_value("temp12"));
    

    /*
    unsafe {
        let hr = winapi::um::objbase::CoInitialize(std::ptr::null_mut());
        assert!(winapi::shared::winerror::SUCCEEDED(hr));

        let mut opc_wrapper: *mut winapi::ctypes::c_void = std::ptr::null_mut();
        let hr =
            winapi::um::combaseapi::CoCreateInstance(
                &gbdaaut::OPCServer::uuidof(),
                std::ptr::null_mut(),
                winapi::um::combaseapi::CLSCTX_ALL,
                &<gbdaaut::IOPCAutoServer as winapi::Interface>::uuidof(),
                &mut opc_wrapper
            );
        println!("Result {}",hr);
        assert!(winapi::shared::winerror::SUCCEEDED(hr));

        let opc_wrapper = &*(opc_wrapper as *mut ::gbdaaut::IOPCAutoServer);

        let server: BSTR = *winrt::BStr::from("Freelance2000OPCServer.202.1").get_address();
        let node: VARIANT = std::mem::zeroed();
        let hr = opc_wrapper.Connect(server, node);
        assert!(winapi::shared::winerror::SUCCEEDED(hr));


        println!("Server {:?}", server);

        let mut major_version: i16 =  0;
        let hr = opc_wrapper.get_MajorVersion(&mut major_version);
        assert!(winapi::shared::winerror::SUCCEEDED(hr));

        println!("Version {:?}", major_version);

        let hr = opc_wrapper.Disconnect();
        assert!(winapi::shared::winerror::SUCCEEDED(hr));
        opc_wrapper.Release();
    }
    */
    println!("Program ends!");
}
