#![no_main]
#![no_std]
#[cfg(target_has_atomic)]

use risc0_zkvm_guest::{env, sha};
use checker_core::MyStruct;
use  signature_bls::{Signature, PublicKey};

risc0_zkvm_guest::entry!(main);
#[macro_use]
extern crate alloc;
pub fn main() {
   /* let my_struct: MyStruct = env::read(); 
    let my_struct_str = format!("{} ", my_struct);
    let my_struct_str = my_struct_str.as_bytes();*/
    let pk: PublicKey = env::read();
    let sign: Signature = env::read();
    let msg: &str = env::read();
    let result = sign.verify(pk, msg);
    //env::commit();  
   // env::commit(&sha::digest_u8_slice(my_struct_str));
}