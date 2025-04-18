use serde::Deserialize;
use std::{
    cmp::{max, min},
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
pub struct MeshFrame {
    vertices: Vec<f64>,
    duration: i32,
}

#[derive(Deserialize, Clone)]
pub struct FFD {
    pub name: String,
    pub frame: Vec<MeshFrame>,
}
#[derive(Deserialize, Clone)]
pub struct Animation {
    pub name: String,
    pub duration: i32,
    pub bone: Vec<AnimBone>,
    pub ffd: Vec<FFD>,
}

#[derive(Deserialize, Clone)]
pub struct Armature {
    pub animation: Vec<Animation>,
    pub bone: Vec<Bone>,
    pub slot: Vec<Slot>,
    pub skin: Vec<Skin>,
}

#[derive(Deserialize, Clone, Default)]
pub struct Slot {
    pub name: String,
    #[serde(default = "i32_default")]
    pub z: i32,
    pub parent: String,
    #[serde(default, rename = "displayIndex")]
    pub display_index: i32,
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

    #[serde(default)]
    triangles: Vec<f64>,

    #[serde(default)]
    vertices: Vec<f64>,
    #[serde(default)]
    uvs: Vec<f64>,
    #[serde(default)]
    edges: Vec<f64>,
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

#[derive(Clone, Copy, Default)]
pub struct Tri {
    pub v1: i32,
    pub v2: i32,
    pub v3: i32,
}

/// Returned from animate(), providing all animation data of a single bone (and then some) in a single frame.
#[derive(Clone)]
pub struct Prop {
    pub name: String,
    pub parent_name: String,

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

    /// Mesh data
    pub is_mesh: bool,
    pub verts: Vec<Vec2>,
    pub tris: Vec<Tri>,
    pub uvs: Vec<Vec2>,

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
fn i32_default() -> i32 {
    return 0;
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
    speed: i32,
) -> Vec<Prop> {
    let mut props: Vec<Prop> = Vec::new();

    for anim_bone in &root.armature[0].animation[anim_idx].bone {
        let bone =
            &root.armature[0].bone[idx_from_name(&anim_bone.name, &root.armature[0].bone) as usize];
        props.push(create_prop(bone, tex, &root.armature[0]));

        let this_prop = props.last_mut().unwrap().clone();
        let mut parent_rot = 0.;

        // inherit transform from parent
        if this_prop.parent_name != "" {
            let parent =
                props[idx_from_name(&props.last().unwrap().parent_name, &props) as usize].clone();
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
            let pos = animate_translate(-parent_rot, &anim_bone.translate_frame, frame, 0, speed);
            props.last_mut().unwrap().pos.x += pos.x;
            props.last_mut().unwrap().pos.y += pos.y;
        }
        if anim_bone.scale_frame.len() > 0 {
            let scale = animate_vec2(&anim_bone.scale_frame, frame, 0, speed);
            props.last_mut().unwrap().scale.x *= scale.x;
            props.last_mut().unwrap().scale.y *= scale.y;
        }
        if anim_bone.rotate_frame.len() > 0 {
            props.last_mut().unwrap().rot +=
                animate_float(&anim_bone.rotate_frame, frame, 0, speed);
        }
    }

    for f in &root.armature[0].animation[anim_idx].ffd {
        let lp = &props.clone();
        let p = &mut props[idx_from_name(&f.name, lp) as usize];
        let mesh_frame = &f.frame;
        if mesh_frame.len() > 0 {
            p.verts = animate_mesh(&mesh_frame, frame, p.clone(), speed);
        }
    }
    props
}

fn create_prop(bone: &Bone, tex: &Texture, armature: &Armature) -> Prop {
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
    let mut slot = &Slot::default();
    let mut skin_slot = &SkinSlot::default();

    // get bone's texture & it's data
    let si = parent_of_slot(&bone.name, &armature.slot);
    if si != -1 {
        slot = &armature.slot[si as usize];
        skin_slot =
            &armature.skin[0].slot[idx_from_name(&slot.name, &armature.skin[0].slot) as usize];

        this_tex_pos = Vec2 {
            x: skin_slot.display[0].transform.x,
            y: skin_slot.display[0].transform.y,
        };

        // get corresponding texture
        this_tex_idx = idx_from_name(&skin_slot.display[0].name, &tex.sub_texture) as usize;
        this_tex = tex.sub_texture[this_tex_idx].clone();
    }

    // format verts, tris and uvs
    let mut f_verts: Vec<Vec2> = vec![];
    let mut f_uvs: Vec<Vec2> = vec![];
    let mut f_tris: Vec<Tri> = vec![];
    let is_mesh = {
        if skin_slot.display.len() > 0 {
            skin_slot.display[0].vertices.len() > 0
        } else {
            false
        }
    };
    if is_mesh {
        let mut i: i32 = 0;
        for v in &skin_slot.display[0].vertices {
            if i % 2 == 0 {
                f_verts.push(Vec2 { x: *v, y: 0. });
            } else {
                f_verts[i as usize / 2].y = *v;
            }
            i += 1;
        }

        i = 0;
        for v in &skin_slot.display[0].uvs {
            if i % 2 == 0 {
                f_uvs.push(Vec2 { x: *v, y: 0. });
            } else {
                f_uvs[i as usize / 2].y = *v;
            }
            i += 1;
        }

        i = 0;
        let mut ki = 0;
        for v in &skin_slot.display[0].triangles {
            if ki == 0 {
                f_tris.push(Tri {
                    v1: *v as i32,
                    v2: 0,
                    v3: 0,
                })
            } else if ki == 1 {
                f_tris[i as usize].v2 = *v as i32;
            } else {
                f_tris[i as usize].v3 = *v as i32;
                i += 1
            }
            ki += 1;
            if ki == 3 {
                ki = 0;
            }
        }
    } else {
        // create basic mesh of 2 tris
        //#[rustfmt::skip]
        {
            f_verts.push(Vec2 {
                x: -this_tex.width as f64 / 2. as f64,
                y: -this_tex.height as f64 / 2. as f64,
            });
            f_verts.push(Vec2 {
                x: this_tex.width as f64 / 2. as f64,
                y: -this_tex.height as f64 / 2. as f64,
            });
            f_verts.push(Vec2 {
                x: -this_tex.width as f64 / 2. as f64,
                y: this_tex.height as f64 / 2. as f64,
            });
            f_verts.push(Vec2 {
                x: this_tex.width as f64 / 2. as f64,
                y: this_tex.height as f64 / 2. as f64,
            });
            f_uvs.push(Vec2 { x: 0., y: 0. });
            f_uvs.push(Vec2 { x: 1., y: 0. });
            f_uvs.push(Vec2 { x: 0., y: 1. });
            f_uvs.push(Vec2 { x: 1., y: 1. });
            f_tris.push(Tri {
                v1: 0,
                v2: 1,
                v3: 2,
            });
            f_tris.push(Tri {
                v1: 1,
                v2: 2,
                v3: 3,
            });
        }
    }

    Prop {
        pos: Vec2 {
            x: bone.transform.x,
            y: bone.transform.y,
        },
        scale: Vec2 {
            x: bone.transform.sc_x,
            y: bone.transform.sc_y,
        },
        rot: bone.transform.rot,

        name: bone.name.to_string(),
        parent_name: bone.parent.clone(),

        tex_idx: this_tex_idx as i32,

        tex_size: Vec2 {
            x: this_tex.width as f64,
            y: this_tex.height as f64,
        },
        tex_pos: this_tex_pos,
        tex_rot: if skin_slot.display.len() > 0 {
            skin_slot.display[0].transform.rot
        } else {
            0.
        },

        z: if this_tex.name != "" { slot.z } else { 0 },

        is_mesh: true,
        verts: f_verts,
        uvs: f_uvs,
        tris: f_tris,
    }
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

fn parent_of_slot(name: &String, slot: &Vec<Slot>) -> i32 {
    let mut i = 0;
    for s in slot {
        // DragonBones ignores undisplayed slots when connecting with bones
        if s.display_index == -1 {
            i += 1;
            continue;
        }
        if s.parent == *name {
            return i;
        }
        i += 1;
    }
    return -1;
}

// animate mesh verts
fn animate_mesh(mesh_frame: &Vec<MeshFrame>, frame: i32, prop: Prop, speed: i32) -> Vec<Vec2> {
    let (frame_idx, curr_frame) = get_frame_idx(mesh_frame, frame, speed);

    // return verts as is, if it's not animated
    if mesh_frame[frame_idx as usize].vertices.len() == 0 {
        return prop.verts;
    }

    let mut verts: Vec<Vec2> = vec![];
    let mut i = 0;
    for _ in &mesh_frame[frame_idx as usize].vertices {
        let vert1 = mesh_frame[frame_idx as usize].vertices[i];
        let vert2 = {
            let v = &mesh_frame[frame_idx as usize + 1].vertices;
            if v.len() > i {
                v[i]
            } else {
                0.
            }
        };

        #[rustfmt::skip]
        let tween = Tweener::linear(vert1, vert2,
            mesh_frame[frame_idx as usize].duration * speed)
        .move_to(curr_frame) as f64;

        if i % 2 == 0 {
            verts.push(Vec2 {
                x: prop.verts[i / 2].x + tween,
                y: 0.,
            });
        } else {
            verts[i / 2].y = prop.verts[i / 2].y + tween;
        }

        i += 1;
    }

    // fill in missing verts
    while verts.len() != prop.verts.len() {
        verts.push(Vec2 {
            x: prop.verts[verts.len()].x,
            y: prop.verts[verts.len()].y,
        })
    }

    verts
}

// animate a frame that returns a Vec2
fn animate_vec2(anim_frame: &Vec<Frame>, frame: i32, frame_rate: i32, speed: i32) -> Vec2 {
    let (frame_idx, curr_frame) = get_frame_idx(anim_frame, frame, speed);

    // give values directly if this is either the only frame, or it's over
    if frame_idx == -1 || anim_frame.len() == 1 {
        return Vec2 {
            x: anim_frame.last().unwrap().x,
            y: anim_frame.last().unwrap().y,
        };
    }

    let next_frame_idx = min(frame_idx as usize + 1, anim_frame.len() - 1);

    Vec2 {
        x: Tweener::linear(
            anim_frame[frame_idx as usize].x,
            anim_frame[next_frame_idx].x,
            anim_frame[frame_idx as usize].duration * speed,
        )
        .move_to(curr_frame) as f64,
        y: Tweener::linear(
            anim_frame[frame_idx as usize].y,
            anim_frame[next_frame_idx].y,
            anim_frame[frame_idx as usize].duration * speed,
        )
        .move_to(curr_frame) as f64,
    }
}

// animate a frame that returns a Vec2
fn animate_translate(
    rot: f64,
    anim_frame: &Vec<Frame>,
    frame: i32,
    frame_rate: i32,
    speed: i32,
) -> Vec2 {
    let (frame_idx, curr_frame) = get_frame_idx(anim_frame, frame, speed);

    // give values directly if this is either the only frame, or it's over
    if frame_idx == -1 || anim_frame.len() == 1 {
        return Vec2 {
            x: anim_frame.last().unwrap().x,
            y: anim_frame.last().unwrap().y,
        };
    }

    let f1 = &anim_frame[frame_idx as usize];
    let f2 = &anim_frame[min(frame_idx as usize + 1, anim_frame.len() - 1)];

    Vec2 {
        x: (Tweener::linear(
            f1.x * (-rot * PI / 180.).cos() + f1.y * -(-rot * PI / 180.).sin(),
            f2.x * (-rot * PI / 180.).cos() + f2.y * -(-rot * PI / 180.).sin(),
            f1.duration * speed,
        )
        .move_to(curr_frame) as f64),
        y: (Tweener::linear(
            f1.x * (-rot * PI / 180.).sin() + f1.y * (-rot * PI / 180.).cos(),
            f2.x * (-rot * PI / 180.).sin() + f2.y * (-rot * PI / 180.).cos(),
            f1.duration * speed,
        )
        .move_to(curr_frame) as f64),
    }
}

// animate a frame that returns a float
fn animate_float(anim_frame: &Vec<Frame>, frame: i32, _frame_rate: i32, speed: i32) -> f64 {
    let (frame_idx, curr_frame) = get_frame_idx(anim_frame, frame, speed);

    // ditto animate_frame
    if frame_idx == -1 || anim_frame.len() == 1 {
        return anim_frame.last().unwrap().rotate;
    }

    let next_frame_idx = min(frame_idx as usize + 1, anim_frame.len() - 1);

    Tweener::linear(
        anim_frame[frame_idx as usize].rotate,
        anim_frame[next_frame_idx].rotate,
        anim_frame[frame_idx as usize].duration * speed,
    )
    .move_to(curr_frame) as f64
}

trait AnimatedFrame {
    fn duration(&self) -> i32;
}
macro_rules! animatedFrame {
    ($type:ty) => {
        impl AnimatedFrame for $type {
            fn duration(&self) -> i32 {
                return self.duration.clone();
            }
        }
    };
}
animatedFrame!(Frame);
animatedFrame!(MeshFrame);

// returns current anim frame, as well as the frame of it
fn get_frame_idx<T: AnimatedFrame>(anim_frame: &Vec<T>, frame: i32, speed: i32) -> (i32, i32) {
    let mut time: i32 = 0;
    let mut i: i32 = 0;
    for f in anim_frame {
        if frame < (time + (f.duration() * speed)) {
            return (i, frame - time);
        };
        time += f.duration() * speed;
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
