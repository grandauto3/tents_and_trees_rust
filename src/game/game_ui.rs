use notan::{
    egui::*,
};
use crate::game::game::State;

pub struct GameUI;

impl GameUI {
    pub fn draw_ui(state: &mut State) -> impl FnOnce(&Context) + '_ {
        |ctx| {
            TopBottomPanel::top("menu").show(ctx, |ui| {
                if ui.button("Save config").clicked() {
                    state.save_config();
                }
            });

            Window::new("Inspector")
                .resizable(true)
                .show(&ctx, |ui| {
                    ui.label("Test");
                }, );
        }
    }
}