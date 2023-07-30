use bevy::{prelude::*, sprite::collide_aabb::collide};

#[derive(Debug, Component)]
pub struct PlayerPlugin;

#[derive(Debug, Component)]
pub struct Player {
    pub speed: f32,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        //
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, player_movement);
    }
}

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    //
    let texture = asset_server.load("player.png");

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                rect: Some(Rect::new(1.5, 1.5, 16.5, 16.5)),
                ..default()
            },
            texture,
            ..default()
        },
        Player { speed: 200.0 },
    ));
}

pub fn player_movement(
    mut players: Query<(&mut Transform, &Player)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, player) in &mut players {
        let move_amount = player.speed * time.delta_seconds();

        if input.pressed(KeyCode::Up) {
            transform.translation.y += move_amount;
        }
        if input.pressed(KeyCode::Down) {
            transform.translation.y -= move_amount;
        }
        if input.pressed(KeyCode::Right) {
            transform.translation.x += move_amount;
        }
        if input.pressed(KeyCode::Left) {
            transform.translation.x -= move_amount;
        }
    }
}
