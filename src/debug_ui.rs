use bevy::prelude::*;
use bevy_ecs_ldtk::LevelSelection;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy_rapier2d::render::DebugRenderContext;

fn level_system(
    mut egui_context: ResMut<EguiContext>,
    mut level_selection: ResMut<LevelSelection>,
    mut debug_render_context: ResMut<DebugRenderContext>,
) {
    let index = if let LevelSelection::Index(index) = *level_selection {
        index
    } else {
        0
    };

    let mut level = index;
    egui::Window::new("level inspect").show(egui_context.ctx_mut(), |ui| {
        ui.add(egui::Slider::new(&mut level, 0..=5).text("level"));
        ui.toggle_value(&mut debug_render_context.enabled, "rapier debug");
    });

    if level != index {
        *level_selection = LevelSelection::Index(level);
    }
}

pub struct DebugUiPlugin;
impl Plugin for DebugUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin).add_system(level_system);
    }
}
