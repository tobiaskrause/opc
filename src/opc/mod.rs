pub mod backend;

use backend::*;
use backend::com::*;

pub trait Connected {
    fn disconnect(&self) -> Box<&NotConnected>;
    fn read_value(&self, name: &str) -> String; 
}

pub trait NotConnected {
    fn open(&self, server_name: &str) -> Box<&Connected>;
}

pub struct OPCServer<'a> {
    opc_backend: Box<OPCAutoServer + 'a>
}

impl <'a> OPCServer<'a> {
    pub fn new_with<T: OPCAutoServer + 'a>(opc_backend: T) -> Box<NotConnected + 'a> {
        Box::new(OPCServer{opc_backend: Box::new(opc_backend)})
    }

    pub fn new() -> Box<NotConnected + 'a> {
        let mut opc_backend = ComOPCServer::new();
        opc_backend.init();
        OPCServer::new_with(opc_backend)
    }
}

impl <'a> NotConnected for OPCServer<'a> {
    fn open(&self, server_name: &str) -> Box<&Connected> {
        self.opc_backend.connect(server_name);
        Box::new(self as &Connected)
    }
}

impl <'a> Connected for OPCServer<'a> {
    fn disconnect(&self) -> Box<&NotConnected> {
        self.opc_backend.disconnect();
        Box::new(self as &NotConnected)
    }

    fn read_value(&self, name: &str) -> String {
        self.opc_backend.read_value(name)
    }
}

impl <'a> Drop for OPCServer<'a> {
    fn drop(& mut self) {
    }
}
