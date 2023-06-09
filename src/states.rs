use bevy::prelude::States;

#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default)]
pub enum AppState {
    #[default]
    LoadingAssets,
    Menu,
    LevelList,
    LoadingLevel,
    InGame,
    LevelComplete,
    Credits,
}
