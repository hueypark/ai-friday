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

### avian2d Physics Rules
These rules prevent common physics bugs. Violating them causes silent failures (no collision events, ghost collisions).

1. **모든 상호작용 entity에 `Collider` 필수**: 충돌/수집 대상은 반드시 `Collider`가 있어야 한다. 통과 가능한 수집 아이템은 `Collider` + `Sensor`를 함께 사용.
2. **Kinematic body는 `LinearVelocity`로 이동**: `Transform`을 직접 수정하면 collision event가 발생하지 않을 수 있다. `LinearVelocity`를 설정하면 avian2d가 이동과 충돌을 모두 처리한다.
3. **충돌 감지에는 `CollidingEntities` 사용**: 충돌을 감지해야 하는 entity(Player, Enemy 등)에 `CollidingEntities::default()`를 spawn 시 추가.
4. **충돌 쿼리에 marker component 필터링**: `check_ground`처럼 충돌 대상을 분류할 때 `With<Platform>` 등 marker component로 필터링해야 의도하지 않은 entity와의 충돌을 방지.
5. **리스폰 시 `LinearVelocity` 초기화 필수**: `Transform`만 리셋하면 이전 속도가 유지되어 리스폰 직후 비정상 이동 발생.

### UI/State 전환 Rules
1. **State 전환 전 UI 업데이트 선행**: `next_state.set()`을 호출하면 현재 프레임에서 system이 중단될 수 있으므로, HUD 등 UI 텍스트를 먼저 갱신한 후 state를 전환해야 한다. early return 대신 flag 패턴 사용.
2. **Camera는 X/Y 양축 추적**: 플랫포머에서 Y축 추적이 없으면 점프 시 Player가 화면 밖으로 나감. lerp factor를 X(0.1)보다 Y(0.08)를 낮게 설정하면 자연스러운 카메라 연출이 된다.

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

### Entity Spawn Checklist
새 entity를 추가할 때 아래 항목을 빠짐없이 확인:

| Entity 유형 | 필수 Component |
|-------------|---------------|
| Platform | `Platform`, `Sprite`, `Transform`, `RigidBody::Static`, `Collider` |
| Player | `Player`, `Sprite`, `Transform`, `RigidBody::Dynamic`, `Collider`, `LockedAxes::ROTATION_LOCKED`, `LinearVelocity`, `Grounded`, `CollidingEntities` |
| Enemy | `Enemy`, `Sprite`, `Transform`, `RigidBody::Kinematic`, `Collider`, `PatrolPath`, `PatrolDirection`, `EnemySpeed`, `LinearVelocity`, `CollidingEntities` |
| Collectible | `Collectible`, `CoinValue`, `Sprite`, `Transform`, `Collider`, `Sensor` |

### Agent Teams 워크플로
병렬 수정 시 module ownership 기반으로 agent를 배정한다:
- **player-dev**: `src/player/` -- 이동, 입력, 점프, 착지, 사망 처리
- **level-dev**: `src/level/` -- 맵, 코인, 카메라
- **enemy-dev**: `src/enemy/` -- AI, 패트롤, 적-플레이어 충돌
- **ui-dev**: `src/ui/` -- 메뉴, HUD, 게임오버

각 agent는 자신의 디렉토리만 수정하며, 수정 후 반드시 `cargo check`로 컴파일을 검증한다.
