use bevy::ecs::schedule::States;

#[derive(States, Debug, Eq, PartialEq, Hash, Clone, Copy, Default)]
pub enum GameState {
    #[default]
    Playing,
    GameOver,
}
