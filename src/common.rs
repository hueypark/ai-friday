use bevy::prelude::*;

// ── Game States ──────────────────────────────────────────────
#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    Menu,
    Playing,
    GameOver,
}

// ── Shared Resources ─────────────────────────────────────────
#[derive(Resource, Default)]
pub struct GameData {
    pub score: u32,
    pub lives: u32,
}

// ── Shared Components ────────────────────────────────────────
#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Platform;

#[derive(Component)]
pub struct Collectible;

// ── Constants ────────────────────────────────────────────────
pub const TILE_SIZE: f32 = 32.0;
pub const PLAYER_SPEED: f32 = 200.0;
pub const JUMP_IMPULSE: f32 = 400.0;
pub const INITIAL_LIVES: u32 = 3;

// ── Messages ────────────────────────────────────────────────
#[derive(Message)]
pub struct PlayerDiedEvent;

#[derive(Message)]
pub struct ScoreChangedEvent(pub u32);
