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
use crate::{
    map::{
        tile::tile::*,
        map::*,
    },
    game::game::*,
};


fn main() -> Result<(), String> {
    const SIZE: usize = 12;

    let map = create_map(SIZE);

    // for row_index in 0..map.row_len() {
    //     for col_index in 0..map.column_len() {
    //         let neighbours = get_neighbours((row_index, col_index), &map);
    //         println!("Row: {row_index}, Column: {col_index}, Tile: {:#?}\n{:#?}", map.get(row_index, col_index).unwrap().get_tile_type(), neighbours);
    //     }
    // }

    draw_map(&map);

    let count_tree = map.elements_row_major_iter().filter(move |x| { *x.get_tile_type() == TileType::TREE }).count();

    println!("Tree spaces are: {count_tree}");

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

