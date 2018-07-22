
pub mod com;

pub trait OPCAutoServer {
    fn init(&mut self);
    fn connect(&self, server_name: &str);
    fn read_value(&self, variable_name: &str) -> String;
    fn disconnect(&self, );
}