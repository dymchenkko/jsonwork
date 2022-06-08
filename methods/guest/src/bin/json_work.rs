#![no_main]
#![no_std]
use risc0_zkvm_guest::{env, sha};
use checker_core::MyStruct;
risc0_zkvm_guest::entry!(main);
#[macro_use]
extern crate alloc;
pub fn main() {
    let my_struct: MyStruct = env::read(); 
    let my_struct_str = format!("{} ", my_struct);
    let my_struct_str = my_struct_str.as_bytes();    
    env::commit(&sha::digest_u8_slice(my_struct_str));
}