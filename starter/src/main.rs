use std::fs;
use sha2::{Sha256, Digest};
use methods::{JSON_WORK_ID, JSON_WORK_PATH};
use risc0_zkvm_host::Prover;
use risc0_zkvm_serde::{from_slice, to_vec};
use tempfile::tempdir;
use checker_core::MyStruct;
#[macro_use]
extern crate alloc;
use std::time::Instant;
use signature_bls::SecretKey;
use  signature_bls::Signature;
use  signature_bls::PublicKey;
use rand::{rngs::OsRng, RngCore};
fn main() {

    let mut hasher = Sha256::new();
    let data = b"Hello worldjiojoijojihygtft t!";
    hasher.update(data);
    // `update` can be called repeatedly and is generic over `AsRef<[u8]>`
    hasher.update("String data");
    // Note that calling `finalize()` consumes hasher
    let mut rand_generator = OsRng {};

    let hash = hasher.finalize();
    rand_generator.next_u32();

    let sk = SecretKey::random(rand_generator).unwrap();
    let signat = Signature::new(&sk, &"data").unwrap();
    let pk = PublicKey::from(&sk);
    let nn = signat.verify(pk, "data");
    println!("Binary hash: {:?}", hash);    let a: MyStruct = MyStruct{
        arr: 125,
    };
    println!("Binary hash: {:?}", nn);
    let temp_dir = tempdir().unwrap();
    let id_path = temp_dir
        .path()
        .join("json_work.id")
        .to_str()   
        .unwrap()
        .to_string();
    fs::write(&id_path, JSON_WORK_ID).unwrap();
    let mut prover = Prover::new(&JSON_WORK_PATH, &id_path).unwrap();
    prover.add_input(to_vec(&pk).unwrap().as_slice()).unwrap();
    prover.add_input(to_vec(&signat).unwrap().as_slice()).unwrap();
    prover.add_input(to_vec(&"data").unwrap().as_slice()).unwrap();

    let now = Instant::now();
    let receipt = prover.run().unwrap();
    let elapsed = now.elapsed();

    let c = receipt.get_seal().unwrap();
   /* let c: Result<risc0_zkvm_core::Digest, _> = from_slice(c);
    let c = c.unwrap();
    let mut result = c.as_slice();
    let c: String = format!("{:?}", c);*/
    println!("Elapsed: {:.3?}", elapsed);
    //println!("I know the factors of {:?}, and I can prove it!", c);
}