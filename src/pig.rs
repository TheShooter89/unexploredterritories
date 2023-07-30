use bevy::{prelude::*, sprite::collide_aabb::collide};

use crate::configs::POKEBALL_SPRITE_PATH;
use crate::player::{Money, Player};

#[derive(Debug, Component)]
pub struct PigPlugin;

#[derive(Debug, Component)]
pub struct Pig {
    pub lifetime: Timer,
}

impl Plugin for PigPlugin {
    fn build(&self, app: &mut App) {
        //
        app.add_systems(Update, (spawn_pig, pig_lifetime));
    }
}

pub fn spawn_pig(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    input: Res<Input<KeyCode>>,
    mut money: ResMut<Money>,
    player: Query<&Transform, With<Player>>,
) {
    //
    if !input.just_pressed(KeyCode::Space) {
        return;
    }

    let player_transform = player.single();

    if money.0 >= 10.0 {
        money.0 -= 10.0;
        info!("Spent $10 on a pig, remaining money: ${:?}", money.0);

        let texture = asset_server.load(POKEBALL_SPRITE_PATH);

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    rect: Some(Rect::new(1.5, 1.5, 16.5, 16.5)),
                    ..default()
                },
                texture,
                transform: *player_transform,
                ..default()
            },
            Pig {
                lifetime: Timer::from_seconds(2.0, TimerMode::Once),
            },
        ));
    }
}

pub fn pig_lifetime(
    mut commands: Commands,
    time: Res<Time>,
    mut pigs: Query<(Entity, &mut Pig)>,
    mut money: ResMut<Money>,
) {
    for (pig_entity, mut pig) in &mut pigs {
        pig.lifetime.tick(time.delta());

        if pig.lifetime.finished() {
            money.0 += 15.0;
            commands.entity(pig_entity).despawn();
            info!("Pig sold for $15! Current money: ${:?}", money.0);
        }
    }
}
