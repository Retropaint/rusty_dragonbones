#[macro_use]
pub mod runtime;

#[cfg(test)]
mod tests {
    use std::{thread, time::Duration};

    use crate::runtime::{animate, load_dragon_bones, DragonBonesRoot, Vec2};

    #[test]
    fn load_armature() {
        let (mut root, tex) = load_dragon_bones("/Users/o/downloads/dragon/dragon_2.zip").unwrap();
        //println!("{}", root.armature[0].bone[1].transform.x);
        let mut props = animate(&mut root, &tex, 0, 30, 0);
        for p in props {
            println!("{}", p.name);
            if p.name == "legR" {
                println!("{} {}", p.pos.x, p.pos.y);
            }
        }
        let mut test: f64 = 0.0;
        let mut props = animate(&mut root, &tex, 0, 0, 60);
    }
}
