use std::{fs::File, io::BufReader};

use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
pub struct Frame {
    #[serde(default, rename = "tweenEasing")]
    pub tween_easing: f64,
    #[serde(default)]
    pub x: f64,
    #[serde(default)]
    pub y: f64,
    #[serde(default)]
    pub rotation: f64,
}

#[derive(Deserialize)]
pub struct Bone {
    pub name: String,
    #[serde(default, rename = "translateFrame")]
    pub translate_frame: Vec<Frame>,
    #[serde(default, rename = "scaleFrame")]
    pub scale_frame: Vec<Frame>,
    #[serde(default, rename = "rotationFrame")]
    pub rotation_frame: Vec<Frame>,
}

#[derive(Deserialize)]
pub struct Animation {
    pub name: String,
    pub bone: Vec<Bone>,
}

#[derive(Deserialize)]
pub struct Armature {
    pub animation: Vec<Animation>,
}

#[derive(Deserialize)]
pub struct Root {
    pub armature: Vec<Armature>,
}

pub fn load_dragon_bones(path: &str) -> std::io::Result<Root> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    //let de = &mut serde_json::Deserializer::from_reader(reader);
    //let s: Root = serde_path_to_error::deserialize(de).expect("");
    let r: Root = serde_json::from_reader(reader).expect("");
    Ok(r)
}

pub fn animate<T>(json: Value, model: T, callback: fn(v: Value, m: T)) {
    callback(json, model);
}
