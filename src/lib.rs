#[macro_use]
pub mod runtime;

#[cfg(test)]
mod tests {
    use std::{thread, time::Duration};

    use crate::runtime::{animate, load_dragon_bones, DragonBonesRoot, Vec2};

    #[test]
    fn load_armature() {
        let mut r: DragonBonesRoot =
            load_dragon_bones("/Users/o/downloads/gopher/gopher.zip").expect("");
        let mut props = animate(&mut r, 0, 30, 0);
        println!("{} {} {}", props[0].rot, props[0].pos.x, props[0].scale.x);
        let mut test: f64 = 0.0;
        let mut props = animate(&mut r, 0, 0, 60);
    }
}
