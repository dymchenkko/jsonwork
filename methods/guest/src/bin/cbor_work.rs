#![no_main]
#![no_std]
use risc0_zkvm_guest::env;
use checker_core::MyStruct;
use cbor_no_std::{ ser::to_bytes, de::from_bytes, value::Value};
risc0_zkvm_guest::entry!(main);
#[macro_use]
extern crate alloc;

pub fn main() {
    let a: u64 = env::read();

    let b: u64 = env::read();

    let data=Value::String(format!("hfuewfwuefhuwhfuewh"));
   
    let mut e = to_bytes(data.clone());
    
    let mut d = from_bytes(e.clone());

    for i in e.iter() {

    }
}