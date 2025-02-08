# rusty_dragonbones
Runtime for [DragonBones](https://dragonbones.github.io/en/index.html) armature animations.

## Installation
```bash
cargo add rusty_dragonbones
```

## Usage
First, load a DragonBones root. This is done via`load_dragonbones()` which requires the _ske and _tex json strings.

Example of extracting said strings from the exported zip via the `zip` crate:
```rust
let path = File::open("/path/to/zip").unwrap();
let mut zip = ZipArchive::new(&path).unwrap();
let mut ske_json = String::new();
let mut tex_json = String::new();
zip.by_index(0).unwrap().read_to_string(&mut ske_json).unwrap();
zip.by_index(2).unwrap().read_to_string(&mut tex_json).unwrap();
let (mut dbroot, dbtex) = load_dragonbones_from_str(&mut ske_json, &mut tex_json).unwrap();
```
From here, animations can be called via `animate()`, which will return a bunch of props.

Example of getting the props for the first animation's first frame:
```rust
let animation_index = 0;
let current_frame = 0.;
let mut props: Vec<Prop> = animate(&mut dbroot, &dbtex, animation_index, current_frame);
```
Props contain the properties of all bones in the animation at their specified frame. They can be used to animate like so:
```rust
// 'props' is the vector from the example above
for p in props {
    draw_image(img, p.pos.x, p.pos.y, p.scale.x, p.scale.y, p.rot);
}
```
This is an oversimplified example. In practice, there'll need to be adjustments depending on the features and/or limitations of your environment.

## Limitations
All of the below will be addressed at some point, but there's no roadmap and this list is not in order of priority, and is not exhaustive.

* Only supports linear tweening
* Only supports one image per bone
* Not much QoL and customization for animating
