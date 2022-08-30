use bevy::prelude::*;

use super::asset::Spritesheet;
#[derive(Component)]
pub struct SpritesheetAnimation {
    pub spritesheet: Handle<Spritesheet>,
    pub active_animation: String,
    pub current_frame: Option<usize>,
    pub frame_timer: Option<Timer>,
    pub do_loop: bool,
    pub end_frame: bool,
}
impl SpritesheetAnimation {
    pub fn new(spritesheet: Handle<Spritesheet>) -> Self {
        Self {
            spritesheet,
            active_animation: default(),
            current_frame: None,
            frame_timer: None,
            do_loop: false,
            end_frame: false,
        }
    }

    pub fn start_animation(&mut self, name: &str, do_loop: bool) {
        debug!("animations: {} -> {}", self.active_animation, name);
        self.active_animation = name.into();
        self.current_frame = None;
        self.frame_timer = None;
        self.do_loop = do_loop;
        self.end_frame = false;
    }
    pub fn is_animation_finished(&self) -> bool {
        self.end_frame
    }
}
