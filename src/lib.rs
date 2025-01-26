pub mod runtime;

#[cfg(test)]
mod tests {
    use std::{thread, time::Duration};

    use crate::runtime::{animate, load_dragon_bones, DragonBonesRoot};

    #[test]
    fn load_armature() {
        let mut r: DragonBonesRoot =
            load_dragon_bones("/Users/o/projects/code/rust/rusty_dragonbones/src/gopher_ske.json")
                .expect("");
        let mut test: f64 = 0.0;
        let props = animate(&mut r, 0, 0, 60);
        println!("{}", props[1].pos.x);
        let mut frame: i32 = 0;
        return;
        loop {
            thread::sleep(Duration::from_millis(100));
            frame += 1;
        }
    }
}
