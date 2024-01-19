use array2d::Array2D;
use notan::{
    AppState,
    draw::{
        CreateDraw,
        DrawShapes,
    },
    prelude::{
        Color,
        Graphics,
    },
    prelude::Assets,
};
use crate::{
    map::{
        map::create_map,
        tile::tile::Tile,
    },
    config::game_config::GameConfig,
};

#[derive(AppState)]
pub struct State {
    map: Array2D<Tile>,
    //pub toml: Asset<String>,
}

impl State {
    pub fn create_game_state(assets: &mut Assets) -> Self {
        // match read_file_as_string("config.toml") {
        //     Ok(s) => { println!("{}", s) }
        //     Err(e) => { println!("{}", e.to_string()) }
        // };

        let cfg = GameConfig::load_or_create_new();
        //keep this line to save a new config.toml
        //ToDo: add mechanism to save on demand
        //GameConfig::save_config(&cfg);

        Self {
            map: create_map(cfg.model.map_config.size as usize),
            //toml: cfg,
        }
    }

    pub fn get_map(&self) -> &Array2D<Tile> {
        &self.map
    }
}

pub struct Game;

impl Game {
    pub fn draw(gfx: &mut Graphics, state: &mut State) {
        let mut draw = gfx.create_draw();
        draw.clear(Color::TEAL);

        let map_row_len = state.map.row_len();
        let map_col_len = state.map.column_len();

        let mut index = 0;

        for x in 0..map_row_len {
            for y in 0..map_col_len {
                const SIZE: f32 = 50f32;
                const POS_OFFSET: f32 = 10f32;
                const OFFSET: f32 = 100f32;

                let position = (((SIZE + POS_OFFSET) * x as f32) + OFFSET, ((SIZE + POS_OFFSET) * y as f32) + OFFSET);
                let color = match index % 5 {
                    0 => { Color::RED }
                    1 => { Color::BLACK }
                    2 => { Color::BLUE }
                    3 => { Color::PURPLE }
                    4 => { Color::OLIVE }
                    _ => { Color::WHITE }
                };

                draw.rect(position, (SIZE, SIZE)).color(color);

                index += 1;
            }
        }

        gfx.render(&draw);
    }
}