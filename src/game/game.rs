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
};
use notan::prelude::{Asset, Assets};
use crate::map::map::create_map;
use crate::map::tile::tile::Tile;
use crate::resources::resource_handler::{read_file, read_file_as_string};

#[derive(AppState)]
pub struct State {
    map: Array2D<Tile>,
    //pub toml: Asset<String>,
}

impl State {
    pub fn create_game_state(assets: &mut Assets) -> Self {
        match read_file_as_string("config.toml") {
            Ok(s) => { println!("{}", s) }
            Err(e) => { println!("{}", e.to_string()) }
        };

        Self {
            map: create_map(12),
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