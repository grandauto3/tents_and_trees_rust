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
use crate::map::tile::tile::TileType;

#[derive(AppState)]
pub struct State {
    map: Array2D<Tile>,
    //pub toml: Asset<String>,
    cfg: GameConfig,

    pub offset: (f32, f32),
}

impl State {
    pub fn create_game_state(assets: &mut Assets) -> Self {
        let cfg = GameConfig::load_or_create_new();

        let size = cfg.model.map_config.get_size();

        Self {
            map: create_map(size),
            cfg,
            offset: (20f32, 20f32),
        }
    }

    pub fn save_config(&self) {
        GameConfig::save_config(&self.cfg);
    }

    pub fn get_map(&self) -> &Array2D<Tile> {
        &self.map
    }

    pub fn get_map_size(&self) -> (usize, usize) {
        self.cfg.model.map_config.get_size()
    }
    pub fn set_map_size(&mut self, new: (usize, usize)) {
        self.cfg.model.map_config.set_size(new);
    }
    pub fn redraw_map(&mut self) {
        self.map = create_map(self.get_map_size());
    }
}

pub struct Game;

impl Game {
    pub fn draw(app: &mut App, gfx: &mut Graphics, plugins: &mut Plugins, state: &mut State) {
        let mut draw = gfx.create_draw();
        draw.clear(Color::TEAL);

        let map_row_len = &state.map.row_len();
        let map_col_len = &state.map.column_len();

        let window_size = &app.backend.window().size();
        let window_center = (window_size.0 / 2, window_size.1 / 2);

        const SIZE_OF_CELL: f32 = 50f32;
        const PADDING: f32 = 10f32;
        const OFFSET: f32 = 10f32;
        const CELL_WITH_PADDING: f32 = SIZE_OF_CELL + PADDING;

        let center_of_map_in_x = (map_row_len / 2) as f32;
        let center_of_map_in_y = (map_col_len / 2) as f32;
        let center_of_map_relative = (
            window_center.0 as f32 - (CELL_WITH_PADDING * center_of_map_in_x),
            window_center.1 as f32 - (CELL_WITH_PADDING * center_of_map_in_y)
        );


        for (x, row) in state.map.rows_iter().enumerate() {
            for (y, element) in row.enumerate() {
                let off_set_x = center_of_map_relative.0 + OFFSET;
                let off_set_y = center_of_map_relative.1 + OFFSET;

                let position = (
                    ((CELL_WITH_PADDING) * x as f32) + off_set_x,
                    ((CELL_WITH_PADDING) * y as f32) + off_set_y,
                );

                let color = match element.get_tile_type() {
                    TileType::UNKNOWN => Color::GRAY,
                    TileType::EMPTY => Color::OLIVE,
                    TileType::TENT => Color::YELLOW,
                    TileType::TREE => Color::GREEN,
                    _ => Color::RED,

                };

                draw.rect(position, (SIZE_OF_CELL, SIZE_OF_CELL)).color(color);
            }
        }

        gfx.render(&draw);


        let ui = GameUI::draw_ui(state);
        let ui = plugins.egui(ui);
        gfx.render(&ui);
    }
}