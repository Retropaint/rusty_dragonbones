mod runtime;

pub use runtime::animate;
pub use runtime::load_dragon_bones;

#[cfg(test)]
mod tests {
    use crate::{
        load_dragon_bones,
        runtime::{Armature, Root},
    };

    #[test]
    fn load_armature() {
        let r: Root = load_dragon_bones("").expect("");
        panic!("{}", r.armature[0].animation[0].name);
    }
}
