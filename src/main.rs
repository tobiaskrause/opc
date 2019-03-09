extern crate opc;

use opc::opc::*;

fn main() {
    let opc_server = OPCServer::new();
    let connection = opc_server.open("Graybox.Simulator.1");
    println!("OPC Server variables:");
    let vars = connection.list();
    println!("{}", vars.join("\n"));
    println!("Program ends!");
}
