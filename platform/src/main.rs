extern crate log;
extern crate simple_logger;

use log::{trace};
use ra_common::LifeCycle;
use seda_bus::bus::MessageBus;

fn main() {
    simple_logger::init().unwrap();
    trace!("Starting 1M5 Daemon...");
    let mut bus = MessageBus::new(String::from("1M5"));
    let a_id = bus.create_endpoint();
    let mut end_a = bus.endpoint(a_id);
    let b_id = bus.create_endpoint();
    let mut end_b = bus.endpoint(b_id);
    let c_id = bus.create_endpoint();
    let mut end_c = bus.endpoint(c_id);
    let d_id = bus.create_endpoint();
    let mut end_d = bus.endpoint(d_id);
    bus.start();
    trace!("1M5 Daemon Stopped.");
}