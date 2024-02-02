use notan::egui::*;
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

            const SLIDER_SPEED: f64 = 0.01;

            Window::new("Inspector")
                .resizable(true)
                .show(&ctx, |ui| {
                    ui.label("Size");
                    ui.horizontal(|ui| {
                        ui.label("X");
                        let mut x = state.get_map_size().0;
                        ui.add(DragValue::new(&mut x).speed(SLIDER_SPEED));
                        state.set_map_size((x, state.get_map_size().1));
                    });
                    ui.horizontal(|ui| {
                        ui.label("Y");
                        let mut y = state.get_map_size().1;
                        ui.add(DragValue::new(&mut y).speed(SLIDER_SPEED));
                        state.set_map_size((state.get_map_size().0, y));
                    });
                }, );
        }
    }
}