use serde::Deserialize;
use std::{
    f64::consts::PI,
    fs::File,
    io::{BufReader, Read},
};
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

#[derive(Deserialize, Clone, Default)]
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
    pub slot: Vec<Slot>,
    pub skin: Vec<Skin>,
}

#[derive(Deserialize, Clone)]
pub struct Slot {
    pub name: String,
    #[serde(default)]
    pub z: i32,
}

#[derive(Deserialize, Clone)]
pub struct Skin {
    slot: Vec<SkinSlot>,
    name: String,
}

#[derive(Deserialize, Clone, Default)]
pub struct SkinSlot {
    display: Vec<Display>,
    name: String,
}

#[derive(Deserialize, Clone, Default)]
pub struct Display {
    name: String,
    transform: Transform,
}

#[derive(Deserialize, Clone)]
pub struct DragonBonesRoot {
    pub armature: Vec<Armature>,
    #[serde(default, rename = "frameRate")]
    pub frame_rate: i32,
}

#[derive(Clone, Copy, Default)]
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

    pub tex_idx: i32,

    /// Bone transforms
    pub pos: Vec2,
    pub scale: Vec2,
    pub rot: f64,

    /// Texture transforms
    /// These are for the singular image tied to the bone. Mutliple images in one bone are not supported.
    pub tex_size: Vec2,
    pub tex_pos: Vec2,
    pub tex_rot: f64,

    /// z-index. Lower values should render behind higher.
    pub z: i32,
}

#[derive(Deserialize, Clone, Default)]
pub struct Texture {
    #[serde(default, rename = "SubTexture")]
    pub sub_texture: Vec<SubTexture>,
}

#[derive(Deserialize, Clone)]
pub struct SubTexture {
    #[serde(default, rename = "frameHeight")]
    pub frame_height: i32,
    #[serde(default, rename = "frameWidth")]
    pub frame_width: i32,
    pub width: i32,
    pub height: i32,
    pub x: i32,
    pub y: i32,
    pub name: String,
}

fn transform_default() -> f64 {
    return 9999.0;
}
fn scale_default() -> f64 {
    return 1.0;
}

/// Load a DragonBones model via file paths to the *ske.json and *tex.json.
pub fn load_dragonbones(
    ske_str: &mut String,
    tex_str: &mut String,
) -> std::io::Result<(DragonBonesRoot, Texture)> {
    let mut root: DragonBonesRoot = serde_json::from_str(ske_str).unwrap();
    let tex: Texture = serde_json::from_str(tex_str).unwrap();
    normalize_frames(&mut root.armature[0]);
    Ok((root, tex))
}

// add back missing transform values in anim frames, using their initial ones
fn normalize_frames(armature: &mut Armature) {
    for a in &mut armature.animation {
        for b in &mut a.bone {
            for f in &mut b.translate_frame {
                if f.x == transform_default() {
                    f.x = 0.;
                }
                if f.y == transform_default() {
                    f.y = 0.;
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

/// Animate DragonBones armature with the specified animation and frame data.
pub fn animate(
    root: &mut DragonBonesRoot,
    tex: &Texture,
    anim_idx: usize,
    frame: i32,
    frame_rate: i32,
) -> Vec<Prop> {
    let mut props: Vec<Prop> = Vec::new();

    let mut bi = 0;
    for anim_bone in &root.armature[0].animation[anim_idx].bone {
        let mut this_tex = SubTexture {
            frame_width: 0,
            frame_height: 0,
            width: 0,
            height: 0,
            name: "".to_string(),
            x: 0,
            y: 0,
        };
        let mut this_tex_pos = Vec2::default();
        let mut this_tex_idx = 0;
        let mut sk = &SkinSlot::default();

        // ignore 0 bi since that's the root
        if bi != 0 {
            sk = &root.armature[0].skin[0].slot
                [idx_from_name(&anim_bone.name, &root.armature[0].skin[0].slot) as usize];

            this_tex_pos = Vec2 {
                x: sk.display[0].transform.x,
                y: sk.display[0].transform.y,
            };

            // get corresponding texture
            this_tex_idx = idx_from_name(&sk.display[0].name, &tex.sub_texture) as usize;
            this_tex = tex.sub_texture[this_tex_idx].clone();
        }

        let bone =
            &root.armature[0].bone[idx_from_name(&anim_bone.name, &root.armature[0].bone) as usize];

        props.push(Prop {
            pos: Vec2 {
                x: bone.transform.x,
                y: bone.transform.y,
            },
            scale: Vec2 {
                x: bone.transform.sc_x,
                y: bone.transform.sc_y,
            },
            rot: bone.transform.rot,

            name: anim_bone.name.to_string(),
            parent_name: bone.parent.clone(),
            parent_idx: idx_from_name(&bone.parent, &props),

            tex_idx: this_tex_idx as i32,

            tex_size: Vec2 {
                x: this_tex.width as f64,
                y: this_tex.height as f64,
            },
            tex_pos: this_tex_pos,
            tex_rot: if sk.display.len() > 0 {
                sk.display[0].transform.rot
            } else {
                0.
            },

            z: if bi > 0 {
                root.armature[0].slot
                    [idx_from_name(&anim_bone.name, &root.armature[0].slot) as usize]
                    .z
            } else {
                0
            },
        });

        let this_prop = props.last_mut().unwrap().clone();
        let mut parent_rot = 0.;

        // inherit transform from parent
        if this_prop.parent_name != "" {
            let parent = props[this_prop.parent_idx as usize].clone();
            parent_rot = parent.rot;
            props.last_mut().unwrap().pos.x = this_prop.pos.x * (parent.rot * PI / 180.).cos()
                + this_prop.pos.y * -(parent.rot * PI / 180.).sin();
            props.last_mut().unwrap().pos.y = this_prop.pos.x * (parent.rot * PI / 180.).sin()
                + this_prop.pos.y * (parent.rot * PI / 180.).cos();
            props.last_mut().unwrap().pos.x += parent.pos.x;
            props.last_mut().unwrap().pos.y += parent.pos.y;
            props.last_mut().unwrap().scale.x *= parent.scale.x;
            props.last_mut().unwrap().scale.y *= parent.scale.y;
            props.last_mut().unwrap().rot += parent.rot;
        }

        // animate transforms
        if anim_bone.translate_frame.len() > 0 {
            let pos = animate_pos(-parent_rot, &anim_bone.translate_frame, frame, frame_rate);
            props.last_mut().unwrap().pos.x += pos.x;
            props.last_mut().unwrap().pos.y += pos.y;
        }
        if anim_bone.scale_frame.len() > 0 {
            let scale = animate_vec2(&anim_bone.scale_frame, frame, frame_rate);
            props.last_mut().unwrap().scale.x *= scale.x;
            props.last_mut().unwrap().scale.y *= scale.y;
        }
        if anim_bone.rotate_frame.len() > 0 {
            props.last_mut().unwrap().rot +=
                animate_float(&anim_bone.rotate_frame, frame, frame_rate);
        }

        bi += 1;
    }
    props
}

trait SearchedVector {
    fn name(&self) -> String;
}
macro_rules! searchedVector {
    ($type:ty) => {
        impl SearchedVector for $type {
            fn name(&self) -> String {
                return self.name.clone();
            }
        }
    };
}
searchedVector!(Slot);
searchedVector!(SkinSlot);
searchedVector!(Prop);
searchedVector!(SubTexture);
searchedVector!(Bone);

// helpers to get stuff by name
fn idx_from_name<T: SearchedVector>(name: &String, slot: &Vec<T>) -> i32 {
    let mut i = 0;
    for s in slot {
        if s.name() == *name {
            return i;
        }
        i += 1;
    }
    return -1;
}

// animate a frame that returns a Vec2
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

// animate a frame that returns a Vec2
fn animate_pos(rot: f64, anim_frame: &Vec<Frame>, frame: i32, frame_rate: i32) -> Vec2 {
    let (frame_idx, curr_frame) = get_frame_idx(anim_frame, frame, frame_rate);

    // give values directly if this is either the only frame, or it's over
    if frame_idx == -1 || anim_frame.len() == 1 {
        return Vec2 {
            x: anim_frame.last().unwrap().x,
            y: anim_frame.last().unwrap().y,
        };
    }

    let f1 = &anim_frame[frame_idx as usize];
    let f2 = &anim_frame[frame_idx as usize + 1];

    Vec2 {
        x: (Tweener::linear(
            f1.x * (-rot * PI / 180.).cos() + f1.y * -(-rot * PI / 180.).sin(),
            f2.x * (-rot * PI / 180.).cos() + f2.y * -(-rot * PI / 180.).sin(),
            f1.duration,
        )
        .move_to(curr_frame) as f64),
        y: (Tweener::linear(
            f1.x * (-rot * PI / 180.).sin() + f1.y * (-rot * PI / 180.).cos(),
            f2.x * (-rot * PI / 180.).sin() + f2.y * (-rot * PI / 180.).cos(),
            f1.duration,
        )
        .move_to(curr_frame) as f64),
    }
}

// animate a frame that returns a float
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

/// Prepare texture for rotation
///
/// If the texture's rotation is done via adding up it's own as well as it's bone's, this will help adjust the texture's position such that it will end up in the right place after rotation is applied.
pub fn prep_tex_for_rot(prop: &mut Prop) {
    let rotation = Vec2 {
        x: prop.tex_pos.x * (-prop.tex_rot * PI / 180.).cos()
            + prop.tex_pos.y * -(-prop.tex_rot * PI / 180.).sin(),
        y: prop.tex_pos.x * (-prop.tex_rot * PI / 180.).sin()
            + prop.tex_pos.y * (-prop.tex_rot * PI / 180.).cos(),
    };
    prop.tex_pos = rotation;
}
