
pub mod server;
mod browser;
use std::*;
use std::iter::*;

pub type Result<T> = result::Result<T, String>;
pub type Name = String;

pub trait OPCAutoServer {
    fn connect(&self, server_name: &str) -> Result<()>;
    fn read_value(&self, variable_name: &str) -> Result<String>;
    fn list_names(&self) -> Result<Vec<Name>>;
    fn write_value(&self, variable_name: &str, value: &str) -> Result<()>;
    fn disconnect(&self) -> Result<()>;
}

trait OPCBrowser {
    fn into_iter(self) -> Box<Iterator<Item = Name>>;
}