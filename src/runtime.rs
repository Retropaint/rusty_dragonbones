use std::{fs::File, io::BufReader};

use serde_json::Value;

pub fn load_dragon_bones() -> std::io::Result<Value> {
    let file = File::open("ske.json")?;
    let reader = BufReader::new(file);
    let s: Value = serde_json::from_reader(reader)?;
    Ok(s)
}

pub fn animate<T>(json: Value, model: T, callback: fn(v: Value, m: T)) {
    callback(json, model);
}
