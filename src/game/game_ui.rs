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
                if ui.button("Redraw Map").clicked() {
                    state.redraw_map();
                }
            });

            const SLIDER_SPEED: f64 = 0.01;

            Window::new("Inspector")
                .resizable(true)
                .show(&ctx, |ui| {
                    ui.label("Size");
                    ui.horizontal(|ui| {
                        ui.label("X:");
                        let mut x = state.get_map_size().0;
                        let changed = ui.add(DragValue::new(&mut x).speed(SLIDER_SPEED))
                                        .changed();
                        state.set_map_size((x, state.get_map_size().1));
                        if changed {
                            state.redraw_map();
                        }
                    });
                    ui.horizontal(|ui| {
                        ui.label("Y:");
                        let mut y = state.get_map_size().1;
                        let changed = ui.add(DragValue::new(&mut y).speed(SLIDER_SPEED))
                                        .changed();
                        state.set_map_size((state.get_map_size().0, y));
                        if changed {
                            state.redraw_map();
                        }
                    });
                }, );
        }
    }
}