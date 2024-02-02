mod map;
mod game;
mod resources;
mod config;

use notan::{
    draw::DrawConfig,
    log,
    prelude::{
        Assets,
        WindowConfig,
    },
    egui::*,
};
use crate::game::game::*;


fn main() -> Result<(), String> {
    let window_config = WindowConfig::new()
        .set_resizable(true);

    notan::init_with(State::create_game_state)
        //.add_loader(create_toml_loader())
        .add_config(window_config)
        .add_config(DrawConfig)
        .add_config(log::LogConfig::debug())
        .add_config(EguiConfig)

        .initialize(|assets: &mut Assets, state: &mut State| {})

        .update(|assets: &mut Assets, state: &mut State| {})

        .draw(Game::draw)
        .build()
}

