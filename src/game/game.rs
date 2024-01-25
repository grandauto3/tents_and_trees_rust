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
    app::{App, Plugins},
    egui::EguiPluginSugar,
};
use crate::{
    map::{
        map::create_map,
        tile::tile::Tile,
    },
    config::game_config::GameConfig,
    game::game_ui::GameUI,
};

#[derive(AppState)]
pub struct State {
    map: Array2D<Tile>,
    //pub toml: Asset<String>,
    cfg: GameConfig,
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
            cfg,
        }
    }

    pub fn save_config(&self) {
        GameConfig::save_config(&self.cfg);
    }

    pub fn get_map(&self) -> &Array2D<Tile> {
        &self.map
    }
}

pub struct Game;

impl Game {
    pub fn draw(app: &mut App, gfx: &mut Graphics, plugins: &mut Plugins, state: &mut State) {
        let mut draw = gfx.create_draw();
        draw.clear(Color::TEAL);

        let map_row_len = state.map.row_len();
        let map_col_len = state.map.column_len();

        let window_size = app.backend.window().size();
        let window_center = (window_size.0 / 2, window_size.1 / 2);

        let mut index = 0;
        for x in 0..map_row_len {
            for y in 0..map_col_len {
                const SIZE: f32 = 50f32;
                const PADDING: f32 = 10f32;
                const OFFSET: f32 = 10f32;

                let position = (
                    ((SIZE + PADDING) * x as f32) + OFFSET,
                    ((SIZE + PADDING) * y as f32) + OFFSET
                );

                let off_set_x = window_center.0 as usize / (map_row_len / 2) + SIZE as usize;
                let off_set_y = window_center.1 as usize / (map_col_len / 2) + SIZE as usize;

                let off_set_x = window_center.0 / 2;
                let off_set_y = window_center.1 / 2;

                //println!("offset_x: {}; offset_y: {}", off_set_x, off_set_y);
                //println!("window_x: {}; window_y: {}", window_center.0, window_center.1);

                let position = (
                    ((SIZE + PADDING) * x as f32) + off_set_x as f32,
                    ((SIZE + PADDING) * y as f32) + off_set_y as f32
                );


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


        let ui = GameUI::draw_ui(state);
        let ui = plugins.egui(ui);
        gfx.render(&ui);
    }
}