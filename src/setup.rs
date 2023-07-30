use bevy::{prelude::*, render::camera::ScalingMode};

#[derive(Debug, Component)]
pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        //
        app.add_systems(Startup, load);
    }
}

pub fn load(mut commands: Commands) {
    //
    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 256.0,
        min_height: 144.0,
    };
    commands.spawn(camera);
}
