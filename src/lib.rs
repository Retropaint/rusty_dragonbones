mod runtime;

pub use runtime::animate;
pub use runtime::load_dragon_bones;

#[cfg(test)]
mod tests {
    use std::{thread, time::Duration};

    use crate::{
        animate, load_dragon_bones,
        runtime::{Armature, DragonBonesRoot},
    };

    #[test]
    fn load_armature() {
        let r: DragonBonesRoot =
            load_dragon_bones("/Users/o/projects/code/rust/rusty_dragonbones/src/gopher_ske.json")
                .expect("");
        let test: i32 = 0;
        let mut frame: i32 = 0;
        loop {
            thread::sleep(Duration::from_millis(100));
            animate(&r.armature[0], 0, frame, 60, test, |test, m| {
                for prop in m {
                    println!("{}", prop.pos.x);
                }
            });
            frame += 1;
        }
    }
}
