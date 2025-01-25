pub mod runtime;

#[cfg(test)]
mod tests {
    use std::{thread, time::Duration};

    use crate::runtime::{load_dragon_bones, DragonBonesRoot};

    #[test]
    fn load_armature() {
        let r: DragonBonesRoot =
            load_dragon_bones("/Users/o/projects/code/rust/rusty_dragonbones/src/dragon_ske.json")
                .expect("");
        let mut test: f64 = 0.0;
        println!("{}", r.armature[0].bone[2].transform.rot);
        let mut frame: i32 = 0;
        return;
        loop {
            thread::sleep(Duration::from_millis(100));
            frame += 1;
        }
    }
}
