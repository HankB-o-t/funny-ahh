use serde_json::Value;
use std::fs::File;
use std::io::Read;
use rand::Rng;

pub fn modif() -> String {
    let mut rng = rand::thread_rng();

    let file = File::open("./src/text.json");
    let mut buff = String::new();
    file.expect("errorsito").read_to_string(&mut buff).unwrap();

    let lifemod: Value = serde_json::from_str(&buff).unwrap();

    return lifemod["modif"][rng.gen_range(0..15)].to_string();
}
