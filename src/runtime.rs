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
    pub rotate: f64,
    pub duration: i32,
}

#[derive(Deserialize)]
pub struct Bone {
    pub name: String,
    #[serde(default, rename = "translateFrame")]
    pub translate_frame: Vec<Frame>,
    #[serde(default, rename = "scaleFrame")]
    pub scale_frame: Vec<Frame>,
    #[serde(default, rename = "rotateFrame")]
    pub rotate_frame: Vec<Frame>,
}

#[derive(Deserialize)]
pub struct Animation {
    pub name: String,
    pub duration: i32,
    pub bone: Vec<Bone>,
}

#[derive(Deserialize)]
pub struct Armature {
    pub animation: Vec<Animation>,
}

#[derive(Deserialize)]
pub struct DragonBonesRoot {
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

pub fn load_dragon_bones(path: &str) -> std::io::Result<DragonBonesRoot> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    //let de = &mut serde_json::Deserializer::from_reader(reader);
    //let s: Root = serde_path_to_error::deserialize(de).expect("");
    let r: DragonBonesRoot = serde_json::from_reader(reader).expect("");
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
    if frame > armature.animation[anim_idx].duration {
        return;
    }

    let mut props: Vec<Prop> = Vec::new();
    for bone in &armature.animation[anim_idx].bone {
        props.push(Prop {
            pos: Vec2 { x: 0.0, y: 0.0 },
            scale: Vec2 { x: 0.0, y: 0.0 },
            rot: 1.0,
            name: "".to_string(),
        });
        props.last_mut().unwrap().pos = animate_frame(&bone.translate_frame, frame, frame_rate);
        props.last_mut().unwrap().scale = animate_frame(&bone.scale_frame, frame, frame_rate);
        props.last_mut().unwrap().rot = animate_rotate(&bone.rotate_frame, frame, frame_rate);
    }

    callback(model, props);
}

fn animate_frame(anim_frame: &Vec<Frame>, frame: i32, frame_rate: i32) -> Vec2 {
    let (frame_idx, curr_frame) = get_frame_idx(anim_frame, frame, frame_rate);
    if frame_idx == -1 {
        return Vec2 { x: -1.0, y: -1.0 };
    }

    // since tweener's move_to() only increments in integers, the values are multiplied and then divided by ampl (amplifier)
    let ampl: f64 = 10.0;

    Vec2 {
        x: (Tweener::linear(
            anim_frame[frame_idx as usize].x * ampl,
            anim_frame[(frame_idx + 1) as usize].x * ampl,
            anim_frame[frame_idx as usize].duration,
        )
        .move_to(curr_frame) as f64)
            / ampl,
        y: (Tweener::linear(
            anim_frame[frame_idx as usize].y * ampl,
            anim_frame[(frame_idx + 1) as usize].y * ampl,
            anim_frame[frame_idx as usize].duration,
        )
        .move_to(curr_frame) as f64)
            / ampl,
    }
}

fn animate_rotate(anim_frame: &Vec<Frame>, frame: i32, frame_rate: i32) -> f64 {
    let (frame_idx, curr_frame) = get_frame_idx(anim_frame, frame, frame_rate);
    if frame_idx == -1 {
        return -1.0;
    }

    // since tweener's move_to() only increments in integers, the values are multiplied and then divided by ampl (amplifier)
    let ampl: f64 = 10.0;

    Tweener::linear(
        anim_frame[frame_idx as usize].rotate * ampl,
        anim_frame[(frame_idx + 1) as usize].rotate * ampl,
        anim_frame[frame_idx as usize].duration,
    )
    .move_to(curr_frame) as f64
        / ampl
}

// returns current anim frame, as well as the frame of it
fn get_frame_idx(anim_frame: &Vec<Frame>, frame: i32, frame_rate: i32) -> (i32, i32) {
    let mut time: i32 = 0;
    let mut i: i32 = 0;
    for f in anim_frame {
        if frame < (time + f.duration) {
            return (i, frame - time);
        };
        time += f.duration;
        i += 1;
    }
    (-1, -1)
}
