
extern crate opc;

use opc::opc::backend::server::*;
use opc::opc::backend::*;

const SERVICE_NAME: &str = "Graybox.Simulator.1";

fn get_instance<'a>() -> ComOPCServer<'a> {
    let mut instance = ComOPCServer::new();
    instance.init().unwrap();
    instance
}

fn connect_with_simulator<'a>() -> ComOPCServer<'a> {
    let instance = get_instance();
    instance.connect(SERVICE_NAME).unwrap();
    instance
}

#[test]
fn connect_disconnect_test() {
    let instance = connect_with_simulator();
    instance.disconnect().unwrap();
}

#[test]
fn connect_drop_test() {
    {
        let _instance = connect_with_simulator();
    }
}

#[test]
fn read_success_test() {
    let instance = connect_with_simulator();
    let _value = instance.read_value("test1").unwrap();
    instance.disconnect().unwrap();
}

#[test]
fn read_error_test() {
    let instance = connect_with_simulator();
    let _value = instance.read_value("test1").unwrap();
    instance.disconnect().unwrap();
}

#[test]
fn write_success_test() {
    let instance = connect_with_simulator();
    instance.write_value("test1", "eins").unwrap();
    instance.disconnect().unwrap();
}

#[test]
fn list_names_test() {
    let instance = connect_with_simulator();
    let names = instance.list_names().unwrap();
    instance.disconnect().unwrap();
    assert_eq!(names.len(), 145)
}
