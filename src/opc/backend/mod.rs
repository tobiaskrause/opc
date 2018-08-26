
pub mod com;
use std::*;

pub type Result<T> = result::Result<T, String>;

pub trait OPCAutoServer {
    fn init(&mut self) -> Result<()>;
    fn connect(&self, server_name: &str) -> Result<()>;
    fn read_value(&self, variable_name: &str) -> Result<String>;
    fn write_value(&self, variable_name: &str, value: &str) -> Result<()>;
    fn disconnect(&self) -> Result<()>;
}