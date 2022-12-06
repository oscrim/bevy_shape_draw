# bevy_shape_draw
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