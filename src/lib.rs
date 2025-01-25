mod runtime;

pub use runtime::animate;
pub use runtime::load_dragon_bones;

#[cfg(test)]
mod tests {
    use std::{thread, time::Duration};

    use crate::{
        animate, load_dragon_bones,
        runtime::{Armature, DragonBonesRoot, Prop},
    };

    #[test]
    fn load_armature() {
        let r: DragonBonesRoot =
            load_dragon_bones("/Users/o/projects/code/rust/rusty_dragonbones/src/gopher_ske.json")
                .expect("");
        let mut test: f64 = 0.0;
        let mut frame: i32 = 0;
        loop {
            thread::sleep(Duration::from_millis(100));
            let props: Vec<Prop> = animate(&r.armature[0], 0, frame, 60, &mut test);
            test = props[0].pos.x;
            println!("{}", test);
            frame += 1;
        }
    }
}
