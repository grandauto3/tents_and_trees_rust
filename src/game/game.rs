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
    prelude::Assets
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

        draw.rect((100.0, 100.0), (600.0, 400.0));

        gfx.render(&draw);
    }
}