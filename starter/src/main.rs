use std::fs;
use methods::{JSON_WORK_ID, JSON_WORK_PATH};
use risc0_zkvm_host::Prover;
use risc0_zkvm_serde::{from_slice, to_vec};
use tempfile::tempdir;
use risc0_zkvm_core::Digest;
use checker_core::MyStruct;
#[macro_use]
extern crate alloc;
use std::time::Instant;
fn main() {
    let a: MyStruct = MyStruct{
        arr: 125,
    };
    let temp_dir = tempdir().unwrap();
    let id_path = temp_dir
        .path()
        .join("json_work.id")
        .to_str()   
        .unwrap()
        .to_string();
    fs::write(&id_path, JSON_WORK_ID).unwrap();
    let mut prover = Prover::new(&JSON_WORK_PATH, &id_path).unwrap();
    prover.add_input(to_vec(&a).unwrap().as_slice()).unwrap();

    let now = Instant::now();
    let receipt = prover.run().unwrap();
    let elapsed = now.elapsed();

    let c = receipt.get_seal().unwrap();
    let c: Result<Digest, _> = from_slice(c);
    let c = c.unwrap();
    let c: String = format!("{:?}", c);
    println!("Elapsed: {:.3?}", elapsed);
    println!("I know the factors of {:?}, and I can prove it!", c);
}
