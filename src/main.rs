use bevy::prelude::*;

mod configs;

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

fn render_logo() {
    // UNEXPLORED
    println!("  _    _ _   _ ________   _______  _      ____  _____  ______ _____           ");
    println!(
        " | |  | | \\ | |  ____\\ \\ / /  __ \\| |    / __ \\|  __ \\|  ____|  __ \\          "
    );
    println!(" | |  | |  \\| | |__   \\ V /| |__) | |   | |  | | |__) | |__  | |  | |         ");
    println!(" | |  | | . ` |  __|   > < |  ___/| |   | |  | |  _  /|  __| | |  | |         ");
    println!(" | |__| | |\\  | |____ / . \\| |    | |___| |__| | | \\ \\| |____| |__| |         ");
    println!(
        "  \\____/|_| \\_|______/_/ \\_\\_|    |______\\____/|_|  \\_\\______|_____/          "
    );

    // TERRITORIES
    println!("  _______ ______ _____  _____  _____ _______ ____  _____  _____ ______  _____ ");
    println!(" |__   __|  ____|  __ \\|  __ \\|_   _|__   __/ __ \\|  __ \\|_   _|  ____|/ ____|");
    println!("    | |  | |__  | |__) | |__) | | |    | | | |  | | |__) | | | | |__  | (___  ");
    println!("    | |  |  __| |  _  /|  _  /  | |    | | | |  | |  _  /  | | |  __|  \\___ \\ ");
    println!(
        "    | |  | |____| | \\ \\| | \\ \\ _| |_   | | | |__| | | \\ \\ _| |_| |____ ____) |"
    );
    println!(
        "    |_|  |______|_|  \\_\\_|  \\_\\_____|  |_|  \\____/|_|  \\_\\_____|______|_____/ "
    );
}

fn render_credits() {
    println!("----------------------");
    println!("dedicated to A.");
    println!("made with <3 by tanque");
    println!("----------------------");
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum GameState {
    StartMenu,
    Overworld,
    Interaction,
}

fn main() {
    render_logo();
    render_credits();

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
