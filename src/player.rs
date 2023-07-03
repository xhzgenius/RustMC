use bevy::prelude::*;

/// A "tag" component for all players.
/// A `Player` should also be an `Entity`.
#[derive(Component)]
pub struct Player;

/// A "tag" component for the main player. The main player should also be a `Player`.
/// There should be exactly one main player.
#[derive(Component)]
pub struct MainPlayer;
