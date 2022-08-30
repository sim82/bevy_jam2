use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_ecs_ldtk::LevelSelection;
use bevy_rapier2d::prelude::{AdditionalMassProperties, ExternalImpulse, RigidBody};
use rand::Rng;
use std::time::Duration;

use crate::{assets::MyAssets, Despawn};

use super::resources::FireworkTest;
pub fn explode_firework(mut commands: Commands, pos: Vec2, my_assets: Res<MyAssets>) {
    let mut rng = rand::thread_rng();

    for (radius_range, num) in [(2.0..5.0, 8), (5.0..15.0, 16)] {
        let radius = rng.gen_range(radius_range);
        let color = Color::Hsla {
            hue: rng.gen_range(0.0..360.0),
            saturation: 1.0,
            lightness: 0.5,
            alpha: 1.0,
        };
        for i in 0..num {
            let rad = (i as f32) / (num as f32) * std::f32::consts::PI * 2.0;
            let dir = Quat::from_rotation_z(rad);

            let vel = Vec3::new(1.0, 0.0, 0.0);

            commands
                .spawn_bundle(SpriteBundle {
                    transform: Transform::from_translation(pos.extend(6.0)),
                    texture: my_assets.firework.clone(),
                    sprite: Sprite {
                        color: color.as_rgba(),
                        ..default()
                    },
                    ..default()
                })
                .insert(RigidBody::Dynamic)
                .insert(ExternalImpulse {
                    impulse: dir.mul_vec3(vel).xy() * radius,
                    torque_impulse: 10.0,
                })
                .insert(AdditionalMassProperties::Mass(0.1))
                .insert(Name::new("firework"))
                .insert(Despawn::TimeToLive(1.5));
        }
    }
}

pub fn test_firework_system(
    commands: Commands,
    time: Res<Time>,
    my_assets: Option<Res<MyAssets>>,
    mut firework: ResMut<FireworkTest>,
    level_selection: Res<LevelSelection>,
) {
    firework.timer.tick(time.delta());

    if *level_selection != LevelSelection::Identifier("End".into()) {
        return;
    }

    if let Some(my_assets) = my_assets {
        if firework.timer.just_finished() {
            let mut rng = rand::thread_rng();
            let base = Vec2::new(140.0, 150.0);
            explode_firework(
                commands,
                Vec2::new(rng.gen_range(-50.0..50.0), rng.gen_range(-20.0..20.0)) + base,
                my_assets,
            );

            firework
                .timer
                .set_duration(Duration::from_secs_f32(rng.gen_range(0.2..1.5)));
            // firework.timer.reset();
        }
    }
}
