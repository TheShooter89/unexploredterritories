use bevy::prelude::*;

mod configs;
mod credits;

mod player;
use player::PlayerPlugin;
use player::Money;

mod pig;
use pig::PigPlugin;

mod setup;
use setup::SetupPlugin;

pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const TILE_SIZE: f32 = 0.1;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum GameState {
    StartMenu,
    Overworld,
    Interaction,
}

fn main() {
    credits::print_logo();
    credits::print_credits();

    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: (640.0, 480.0).into(),
                        title: ("unexplored territories".into()),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .insert_resource(Money(100.0))
        .add_plugins((SetupPlugin, PlayerPlugin, PigPlugin))
        .run();
}
