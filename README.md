# bevy_shape_draw
<div align="center">

[![crates.io](https://img.shields.io/crates/v/bevy_shape_draw)](https://crates.io/crates/bevy_shape_draw)
[![docs.rs](https://docs.rs/bevy_shape_draw/badge.svg)](https://docs.rs/bevy_shape_draw)
[![Bevy tracking](https://img.shields.io/badge/Bevy%20tracking-released%20version-lightblue)](https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md#main-branch-tracking)

</div>

A [Bevy](https://github.com/bevyengine/bevy) plugin for drawing a shape using raycasting in 3d space with a mouse. This plugin is build on and relies on [`bevy_mod_raycast`](https://github.com/aevyrie/bevy_mod_picking).

The only shape that can be drawn at the moment is a box of fixed height

Add the plugin to the `[dependencies]` in `Cargo.toml`

```toml
bevy_shape_draw = "0.1"
```

You will need to add the Draw Shape Plugin.

```rust
.add_plugin(bevy_shape_draw::DrawShapePlugin)
```

Then you will have to add the raycast source to your camera.

```rust
.insert(bevy_shape_draw::ShapeDrawRaycastSource::new())
```

Finally, mark any meshes that you want to be able to draw shapes on.

```rust
.insert(bevy_shape_draw::ShapeDrawRaycastMesh::default())
```

## Example

```shell
cargo run --example simple
```

```shell
cargo run --example events
```