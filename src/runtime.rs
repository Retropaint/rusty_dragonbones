use std::{fs::File, io::BufReader};

use serde::Deserialize;
use tween::Tweener;

#[derive(Deserialize, Clone)]
pub struct Frame {
    #[serde(default, rename = "tweenEasing")]
    pub tween_easing: f64,
    #[serde(default = "transform_default")]
    pub x: f64,
    #[serde(default = "transform_default")]
    pub y: f64,
    #[serde(default = "transform_default")]
    pub rotate: f64,
    pub duration: i32,
}

#[derive(Deserialize, Clone)]
pub struct AnimBone {
    pub name: String,
    #[serde(default, rename = "translateFrame")]
    pub translate_frame: Vec<Frame>,
    #[serde(default, rename = "scaleFrame")]
    pub scale_frame: Vec<Frame>,
    #[serde(default, rename = "rotateFrame")]
    pub rotate_frame: Vec<Frame>,
}

#[derive(Deserialize, Clone)]
pub struct Bone {
    pub name: String,
    #[serde(default)]
    pub parent: String,
    pub transform: Transform,
}

#[derive(Deserialize, Clone)]
pub struct Transform {
    #[serde(default)]
    pub x: f64,
    #[serde(default)]
    pub y: f64,

    /// this will probably backfire later
    #[serde(default, rename = "skX")]
    pub rot: f64,

    #[serde(default = "scale_default", rename = "scX")]
    pub sc_x: f64,
    #[serde(default = "scale_default", rename = "scY")]
    pub sc_y: f64,
}

#[derive(Deserialize, Clone)]
pub struct Animation {
    pub name: String,
    pub duration: i32,
    pub bone: Vec<AnimBone>,
}

#[derive(Deserialize, Clone)]
pub struct Armature {
    pub animation: Vec<Animation>,
    pub bone: Vec<Bone>,
}

#[derive(Deserialize, Clone)]
pub struct DragonBonesRoot {
    pub armature: Vec<Armature>,
    #[serde(default, rename = "frameRate")]
    pub frame_rate: i32,
}

#[derive(Clone)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

/// Returned from animate(), providing all animation data of a single bone (and then some) in a single frame.
#[derive(Clone)]
pub struct Prop {
    pub name: String,
    pub parent_name: String,

    /// index of the parent prop, relative to the vector containing this prop and it.
    pub parent_idx: i32,

    pub pos: Vec2,
    pub scale: Vec2,
    pub rot: f64,
}

/// Parameters: (prop1: Prop, prop2: Prop)
/// Inherits transform values from prop2 into prop1.
/// Useful for inheriting parent values into its child/children.
macro_rules! inherit_prop {
    ($prop1:expr, $prop2:expr) => {
        $prop1.pos.x += $prop2.pos.x;
        $prop1.pos.y += $prop2.pos.y;
        $prop1.scale.x *= $prop2.scale.x;
        $prop1.scale.y *= $prop2.scale.y;
        $prop1.rot += $prop2.rot;
    };
}

fn transform_default() -> f64 {
    return 9999.0;
}
fn scale_default() -> f64 {
    return 1.0;
}

pub fn load_dragon_bones(path: &str) -> std::io::Result<DragonBonesRoot> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    //let de = &mut serde_json::Deserializer::from_reader(reader);
    //let s: Root = serde_path_to_error::deserialize(de).expect("");
    let mut r: DragonBonesRoot = serde_json::from_reader(reader).expect("");
    normalize_frames(&mut r.armature[0]);
    Ok(r)
}

/// Add back missing transform values in anim frames, using their initial ones.
fn normalize_frames(armature: &mut Armature) {
    for a in &mut armature.animation {
        for b in &mut a.bone {
            for f in &mut b.translate_frame {
                if f.x == transform_default() {
                    f.x = 0.0;
                }
                if f.y == transform_default() {
                    f.y = 0.0;
                }
            }
            for f in &mut b.scale_frame {
                if f.x == transform_default() {
                    f.x = 1.0;
                }
                if f.y == transform_default() {
                    f.y = 1.0;
                }
            }
            for f in &mut b.rotate_frame {
                if f.rotate == transform_default() {
                    f.rotate = 1.0;
                }
            }
        }
    }
}

/// Animate dragon bones armature with the specified animation and frame data.
pub fn animate(
    root: &mut DragonBonesRoot,
    anim_idx: usize,
    frame: i32,
    frame_rate: i32,
) -> Vec<Prop> {
    let mut props: Vec<Prop> = Vec::new();

    let mut bi = 0;
    for bone in &root.armature[0].animation[anim_idx].bone {
        props.push(Prop {
            pos: Vec2 {
                x: root.armature[0].bone[bi].transform.x,
                y: root.armature[0].bone[bi].transform.y,
            },
            scale: Vec2 {
                x: root.armature[0].bone[bi].transform.sc_x,
                y: root.armature[0].bone[bi].transform.sc_y,
            },
            rot: root.armature[0].bone[bi].transform.rot,

            name: bone.name.to_string(),
            parent_name: root.armature[0].bone[bi].parent.clone(),
            parent_idx: prop_by_name(root.armature[0].bone[bi].parent.clone(), &props),
        });

        // animate transforms
        if bone.translate_frame.len() > 0 {
            let pos = animate_vec2(&bone.translate_frame, frame, frame_rate);
            props.last_mut().unwrap().pos.x += pos.x;
            props.last_mut().unwrap().pos.y += pos.y;
        }
        if bone.scale_frame.len() > 0 {
            let scale = animate_vec2(&bone.scale_frame, frame, frame_rate);
            props.last_mut().unwrap().scale.x *= scale.x;
            props.last_mut().unwrap().scale.y *= scale.y;
        }
        if bone.rotate_frame.len() > 0 {
            props.last_mut().unwrap().rot += animate_float(&bone.rotate_frame, frame, frame_rate);
        }

        // inherit transform from parent
        let this_prop = props.last_mut().unwrap().clone();
        if this_prop.parent_name != "" {
            inherit_prop!(
                props.last_mut().unwrap(),
                props[this_prop.parent_idx as usize]
            );
        }

        bi += 1;
    }
    props
}

/// Return index of prop with this name.
fn prop_by_name(name: String, props: &Vec<Prop>) -> i32 {
    let mut i = 0;
    for p in props {
        if p.name == name {
            return i;
        }
        i += 1;
    }
    return -1;
}

/// Animate a frame that returns a Vec2.
fn animate_vec2(anim_frame: &Vec<Frame>, frame: i32, frame_rate: i32) -> Vec2 {
    let (frame_idx, curr_frame) = get_frame_idx(anim_frame, frame, frame_rate);

    // give values directly if this is either the only frame, or it's over
    if frame_idx == -1 || anim_frame.len() == 1 {
        return Vec2 {
            x: anim_frame.last().unwrap().x,
            y: anim_frame.last().unwrap().y,
        };
    }

    Vec2 {
        x: (Tweener::linear(
            anim_frame[frame_idx as usize].x,
            anim_frame[(frame_idx + 1) as usize].x,
            anim_frame[frame_idx as usize].duration,
        )
        .move_to(curr_frame) as f64),
        y: (Tweener::linear(
            anim_frame[frame_idx as usize].y,
            anim_frame[(frame_idx + 1) as usize].y,
            anim_frame[frame_idx as usize].duration,
        )
        .move_to(curr_frame) as f64),
    }
}

/// Animate a frame that returns a float.
fn animate_float(anim_frame: &Vec<Frame>, frame: i32, _frame_rate: i32) -> f64 {
    let (frame_idx, curr_frame) = get_frame_idx(anim_frame, frame, _frame_rate);

    // ditto animate_frame
    if frame_idx == -1 || anim_frame.len() == 1 {
        return anim_frame.last().unwrap().rotate;
    }

    Tweener::linear(
        anim_frame[frame_idx as usize].rotate,
        anim_frame[(frame_idx + 1) as usize].rotate,
        anim_frame[frame_idx as usize].duration,
    )
    .move_to(curr_frame) as f64
}

// returns current anim frame, as well as the frame of it
fn get_frame_idx(anim_frame: &Vec<Frame>, frame: i32, _frame_rate: i32) -> (i32, i32) {
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
