# ~~Bevy Glowy Orb Tutorial~~ A cube with flat geometric normals made from a mesh with no normals.

`let N = normalize(cross(dpdy(in.world_position.xyz), dpdx(in.world_position.xyz)));`

Depends on bevy 0.14

Use `--features bevy/file_watcher` for hot reloading.

![demo](demo.jpg)
