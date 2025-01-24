use std::{fs::File, io::BufReader};

use serde::Deserialize;
use tween::Tweener;

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
    pub duration: i32,
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
    #[serde(default, rename = "frameRate")]
    pub frame_rate: i32,
}

pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

pub struct Prop {
    pub pos: Vec2,
    pub scale: Vec2,
    pub rot: f64,
    pub name: String,
}

pub fn load_dragon_bones(path: &str) -> std::io::Result<Root> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    //let de = &mut serde_json::Deserializer::from_reader(reader);
    //let s: Root = serde_path_to_error::deserialize(de).expect("");
    let r: Root = serde_json::from_reader(reader).expect("");
    Ok(r)
}

pub fn animate<T>(
    armature: &Armature,
    anim_idx: usize,
    frame: i32,
    frame_rate: i32,
    model: T,
    callback: fn(m: T, p: Vec<Prop>),
) {
    let mut props: Vec<Prop> = Vec::new();

    for bone in &armature.animation[anim_idx].bone {
        animate_frame(&bone.translate_frame, frame, frame_rate, &mut props);
    }

    callback(model, props);
}

fn animate_frame(anim_frame: &Vec<Frame>, frame: i32, frame_rate: i32, props: &mut Vec<Prop>) {
    let frame_idx = get_frame_idx(anim_frame, frame, frame_rate);
    props.push(Prop {
        pos: Vec2 {
            x: (Tweener::linear(0, 20, 600).move_to(frame) as f64) / 10.0,
            y: 0.0,
        },
        scale: Vec2 { x: 0.0, y: 0.0 },
        rot: 1.0,
        name: "".to_string(),
    });
}

fn get_frame_idx(anim_frame: &Vec<Frame>, frame: i32, frame_rate: i32) {
    let time: i32 = 0;
    for f in anim_frame {
        if frame < (time + f.duration) {}
    }
}
