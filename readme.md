# rusty_dragonbones
Runtime for DragonBones animations.

## Installation
```bash
cargo add rusty_dragonbones
```

## Usage
First, load a DragonBones root:
```rust
use rusty_dragonbones::runtime::{load_dragon_bones};

let root: DragonBonesRoot = load_dragon_bones("/path/to/*ske.json").expect("");
```
From here, animations can be called via `animate()`, which will return a bunch of props:
```rust
use rusty_dragonbones::runtime::{animate};

let mut props: Vec<Prop> = animate(&dbroot, 0, game_frame);
```
Props contain the properties of all bones in the animation at their specified frame. They can be used to animate like so:
```rust
let mut props: Vec<Prop> = animate(&dbroot, 0, game_frame);
for p in props {
    draw_image(img, p.pos.x, p.pos.y, p.scale.x, p.scale.y, p.rot);
}
```
This is a simplified example. In practice, you will most likely have to adjust the values here and there. 

## Missing Features
* Only does linear tweening
* Only supports one image per bone
