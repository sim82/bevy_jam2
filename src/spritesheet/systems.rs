use std::time::Duration;

use super::{asset::Spritesheet, components::SpritesheetAnimation};
use bevy::prelude::*;

pub fn spritesheet_animation_system(
    spritesheets: Res<Assets<Spritesheet>>,
    time: Res<Time>,
    mut query: Query<(&mut SpritesheetAnimation, &mut TextureAtlasSprite)>,
) {
    for (mut spritesheet_animation, mut texture_atlas_sprite) in &mut query {
        let has_current_frame = spritesheet_animation.current_frame.is_some();
        if let Some(frame_timer) = spritesheet_animation.frame_timer.as_mut() {
            frame_timer.tick(time.delta());

            if has_current_frame && !frame_timer.just_finished() {
                continue;
            }
        }
        spritesheet_animation.frame_timer = None;

        let spritesheet = spritesheets
            .get(&spritesheet_animation.spritesheet)
            .unwrap();

        let range = spritesheet
            .ranges
            .get(&spritesheet_animation.active_animation)
            .unwrap();
        spritesheet_animation.current_frame = match spritesheet_animation.current_frame {
            Some(mut current_frame) => {
                if current_frame == *range.end() {
                    if spritesheet_animation.do_loop {
                        current_frame = *range.start();
                    }
                    spritesheet_animation.end_frame = true;
                } else {
                    current_frame += 1;
                }
                // if !range.contains(&current_frame) {
                //     current_frame = range.start;
                // }
                Some(current_frame)
            }
            None => Some(*range.start()),
        };
        debug!("current_frame: {:?}", spritesheet_animation.current_frame);
        texture_atlas_sprite.index = spritesheet_animation.current_frame.unwrap();
        if !spritesheet_animation.end_frame || spritesheet_animation.do_loop {
            spritesheet_animation.frame_timer = Some(Timer::new(
                Duration::from_millis(
                    spritesheet.durations[spritesheet_animation.current_frame.unwrap()],
                ),
                false,
            ));
        }
    }
}
