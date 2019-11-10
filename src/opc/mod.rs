pub mod backend;

use self::backend::*;
use self::backend::com::server::*;

pub trait Connected {
    fn disconnect(&self) -> &dyn NotConnected;
    fn read_value(&self, name: &str) -> String; 
    fn list(&self) -> Vec<String>;
}

pub trait NotConnected {
    fn open(&self, server_name: &str) -> &dyn Connected;
}

pub struct OPCServer {
    opc_backend: Box<dyn OPCAutoServer>
}

impl OPCServer {
    pub fn new_with(opc_backend: Box<dyn OPCAutoServer>) -> Box<dyn NotConnected> {
        Box::new(OPCServer{opc_backend})
    }

    pub fn new() -> Box<dyn NotConnected> {
        let opc_backend: Box<dyn OPCAutoServer> = Box::new(ComOPCServer::try_new().unwrap());
        OPCServer::new_with(opc_backend)
    }
}

impl NotConnected for OPCServer {
    fn open(&self, server_name: &str) -> &dyn Connected {
        self.opc_backend.connect(server_name).unwrap();
        self
    }
}

impl Connected for OPCServer {
    fn disconnect(&self) -> &dyn NotConnected {
        self.opc_backend.disconnect().unwrap();
        self
    }

    fn read_value(&self, name: &str) -> String {
        self.opc_backend.read_value(name).unwrap()
    }

    fn list(&self) -> Vec<String> {
        self.opc_backend.list_names().unwrap()
    }
}

impl Drop for OPCServer {
    fn drop(& mut self) {
    }
}
