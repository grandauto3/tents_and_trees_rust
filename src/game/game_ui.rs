use notan::egui::*;
use crate::game::game::State;

pub struct GameUI;

impl GameUI {
    pub fn draw_ui(state: &mut State) -> impl FnOnce(&Context) + '_ {
        |ctx| {
            TopBottomPanel::top("menu").show(ctx, |ui| {
                ui.horizontal(|ui| {
                    if ui.button("Save config").clicked() {
                        state.save_config();
                    }
                    if ui.button("Redraw Map").clicked() {
                        state.redraw_map();
                    }
                });
            });

            const SLIDER_SPEED_SIZE: f64 = 0.01;
            const SLIDER_SPEED_POS: f64 = 0.1;

            Window::new("Inspector")
                .resizable(true)
                .show(&ctx, |ui| {
                    ui.scope(|ui| {
                        ui.heading("Size");
                        ui.horizontal(|ui| {
                            ui.label("X:");
                            let mut x = state.get_map_size().0;
                            let changed = ui.add(DragValue::new(&mut x).speed(SLIDER_SPEED_SIZE))
                                            .changed();
                            state.set_map_size((x, state.get_map_size().1));
                            if changed {
                                state.redraw_map();
                            }
                        });
                        ui.horizontal(|ui| {
                            ui.label("Y:");
                            let mut y = state.get_map_size().1;
                            let changed = ui.add(DragValue::new(&mut y).speed(SLIDER_SPEED_SIZE))
                                            .changed();
                            state.set_map_size((state.get_map_size().0, y));
                            if changed {
                                state.redraw_map();
                            }
                        });
                    });
                    ui.separator();
                    ui.scope(|ui| {
                        ui.heading("Position");
                        ui.horizontal(|ui| {
                            ui.label("X:");
                            let changed = ui.add(DragValue::new(&mut state.offset.0).speed(SLIDER_SPEED_POS))
                                            .changed();
                            if changed {
                                state.redraw_map();
                            }
                        });
                        ui.horizontal(|ui| {
                            ui.label("Y:");
                            let changed = ui.add(DragValue::new(&mut state.offset.1).speed(SLIDER_SPEED_POS))
                                            .changed();
                            if changed {
                                state.redraw_map();
                            }
                        });
                    });
                }, );
        }
    }
}