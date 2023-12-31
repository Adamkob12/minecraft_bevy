# Minecraft with Bevy (Check out the re-iteration: [NovaCraft](https://github.com/Adamkob12/NovaCraft.git))

/
/
/

## Archived in favour of [NovaCraft](https://github.com/Adamkob12/NovaCraft.git), a more recent, well organized, stabler iteration of the project.
/
/
/

## After Bevy 0.12 and [Bevy Meshem](https://github.com/Adamkob12/bevy_meshem) 0.3 update, Final version.
![Screenshot 2023-11-05 at 16 41 53](https://github.com/Adamkob12/minecraft_bevy/assets/46227443/7c91ff54-218c-44fc-acea-c1cb03b4438b)

### You're welcome to take a look!
```Bash
git clone https://github.com/Adamkob12/minecraft_bevy.git
cd minecraft_bevy
cargo run --release
```

## History of development:

![Screenshot 2023-10-11 at 0 35 01](https://github.com/Adamkob12/minecraft_bevy/assets/46227443/ce7d8eaa-d3d9-4735-a3a0-59340525db51)
### Updated after bevy_meshem 0.2.3
![Screenshot 2023-10-14 at 19 21 30](https://github.com/Adamkob12/minecraft_bevy/assets/46227443/330e6711-b566-408d-8543-0827b8e6d85f)
### After adding Ambient Occlusion
![Screenshot 2023-10-15 at 2 35 03](https://github.com/Adamkob12/minecraft_bevy/assets/46227443/ee175b26-ba70-49d3-85ce-429a76fde4cf)

## Minecraft_bevy was built to showcase [bevy_meshem](https://github.com/Adamkob12/bevy_meshem). After a week of developing it has:
### ***Chunk loading / unloading*** 
each chunk's mesh is being quickly generated by bevy_meshem, generating up to thousands of meshes per frame asynchronously. 
### ***Block placing / breaking*** 
using bevy_meshem's 0.2 release that made run-time mesh updates possible, block placing and breaking feels smooth and snappy. Press `Q` to switch blocks. and the numbers `1-9` to select a block.
### ***Custom Collision physics*** 
without any 3rd party crates / physics engines.

## Video Showcase (slightly outdated):

***Most Recent Update: breaking / placing blocks, collision with terrain***
https://github.com/Adamkob12/minecraft_bevy/assets/46227443/fc8418a2-cd44-4f5c-aeb5-3129baca8700

***Terrain generation, run-time chunk loading/unloading:***
https://github.com/Adamkob12/minecraft_bevy/assets/46227443/f3c7828b-7142-44b5-ba03-2fd1a810d524

