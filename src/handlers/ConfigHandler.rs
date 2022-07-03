extern crate serde_json;
use std::fs;
use serde_json::{Value};

#[path="./StructureHandler.rs"]
mod structure_handler;

pub fn cfg() -> Value {
    let data = fs::read_to_string(structure_handler::config_path()).expect("read error");
    let json: serde_json::Value = serde_json::from_str(&data).expect("deserialize error");
    return json;
}