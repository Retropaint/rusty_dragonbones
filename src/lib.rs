mod runtime;

pub use runtime::animate;
pub use runtime::load_dragon_bones;

#[cfg(test)]
mod tests {
    use crate::{
        animate, load_dragon_bones,
        runtime::{Armature, Root},
    };

    #[test]
    fn load_armature() {
        let r: Root =
            load_dragon_bones("/Users/o/projects/code/rust/rusty_dragonbones/src/gopher_ske.json")
                .expect("");
        let test: i32 = 0;
        animate(&r.armature[0], 0, 150, 60, test, |test, m| {
            for prop in m {
                println!("{}", prop.pos.x);
            }
        });
        panic!("");
    }
}
