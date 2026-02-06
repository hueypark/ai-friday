# AI Friday -- 2D Platformer

## Project Overview
A 2D platformer game built with Rust, Bevy 0.15, and avian2d 0.2 physics.

## Build Commands
- `cargo check` -- Verify compilation (run after every change)
- `cargo run` -- Launch the game
- `cargo clippy` -- Lint

## Architecture
Bevy ECS with a plugin-per-module pattern.

### Module Structure
- `src/common.rs` -- **SHARED CONTRACT.** All shared types. DO NOT MODIFY without lead approval.
- `src/main.rs` -- App entry point. DO NOT MODIFY without lead approval.
- `src/player/` -- Player movement, input, physics. Owned by player-dev.
- `src/level/` -- Level layout, tiles, collectibles, camera. Owned by level-dev.
- `src/enemy/` -- Enemy AI, patrol, collisions. Owned by enemy-dev.
- `src/ui/` -- Menus, HUD, game over. Owned by ui-dev.

### Conventions
1. **One Plugin per module**: Each `mod.rs` defines a `pub struct XPlugin` implementing `Plugin`.
2. **State-scoped systems**: Use `.run_if(in_state(GameState::Playing))`.
3. **OnEnter/OnExit**: Use for spawn/despawn.
4. **Import shared types**: `use crate::common::*;`
5. **Physics via avian2d**: `RigidBody::Static` for platforms, `RigidBody::Dynamic` for player, `RigidBody::Kinematic` for enemies.
6. **Colored rectangles for v1**: Use `Sprite { color, custom_size, ..default() }` instead of image assets.
7. **Verify compilation**: Run `cargo check` after every meaningful change.
8. **Each teammate ONLY edits files in their assigned directory.**

### GameState Flow
```
Menu --> Playing --> GameOver
  ^                    |
  +--------------------+
```

### Key Shared Types (from common.rs)
- `GameState` -- Menu, Playing, GameOver
- `GameData` -- Resource with score, lives
- `Player`, `Enemy`, `Platform`, `Collectible` -- marker components
- `PlayerDiedEvent`, `ScoreChangedEvent` -- events
- Constants: `TILE_SIZE`, `PLAYER_SPEED`, `JUMP_IMPULSE`, `INITIAL_LIVES`

### Player Input Resource (from player/components.rs)
- `PlayerInput` -- Resource with `move_direction: f32` and `jump_pressed: bool`
