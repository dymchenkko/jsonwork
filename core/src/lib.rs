#![no_std]
extern crate alloc;
use serde::{Deserialize, Serialize};
use core::str;

#[derive(Deserialize, Serialize)]
pub struct MyStruct {
    pub arr: u64,
}
impl core::fmt::Display for MyStruct {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        serde_json_core_fmt::to_fmt(f, self)
    }
}

#[derive(Serialize)]
pub struct Res {
    pub res: str,
}

