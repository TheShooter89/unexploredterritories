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
