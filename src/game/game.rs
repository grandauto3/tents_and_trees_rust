use array2d::Array2D;
use notan::{
    AppState,
    draw::{
        CreateDraw,
        DrawShapes,
    }, prelude::{
        Color,
        Graphics,
    }, app::{
        App, Plugins,
    },
    egui::EguiPluginSugar,
};
use crate::{
    map::{
        map::create_map,
        tile::{
            tile::{
                Tile,
                TileType,
            }
        },
    },
    config::{
        game_config::GameConfig,
        model::game_ui_config::GameUiConfig,
    },
    game::{
        game_ui::GameUI,
        game_state::GameState,
    },
};

#[derive(AppState)]
pub struct State {
    map: Array2D<Tile>,
    cfg: GameConfig,
    ui_model: GameUiConfig,
    game_state: GameState,
}

impl State {
    pub fn create_game_state() -> Self {
        let cfg = GameConfig::load_or_create_new();

        let map_size = cfg.model.map_config.get_map_size();
        let tile_size = cfg.model.map_config.get_tile_size();

        Self {
            map: create_map(map_size, tile_size),
            cfg,
            ui_model: GameUiConfig::default(),
            game_state: GameState::default(),
        }
    }

    pub fn save_config(&self) {
        GameConfig::save_config(&self.cfg);
    }

    pub fn get_map(&self) -> &Array2D<Tile> {
        &self.map
    }

    pub fn get_map_size(&self) -> (usize, usize) {
        self.cfg.model.map_config.get_map_size()
    }

    pub fn set_map_size(&mut self, new: (usize, usize)) {
        self.cfg.model.map_config.set_size(new);
    }

    pub fn get_tile_size(&self) -> (f32, f32) {
        self.cfg.model.map_config.get_tile_size()
    }

    pub fn regenerate_map(&mut self) {
        self.map = create_map(self.get_map_size(), self.get_tile_size());
    }

    pub fn get_game_ui_config(&mut self) -> &mut GameUiConfig {
        &mut self.ui_model
    }
}

pub struct Game;

impl Game {
    pub fn update(app: &mut App, state: &mut State) {
        Self::calculate_tile_pos(app, state);
        Self::process_input(app, state);
    }

    pub fn draw(app: &mut App, gfx: &mut Graphics, plugins: &mut Plugins, state: &mut State) {
        let mut draw = gfx.create_draw();
        draw.clear(Color::TEAL);

        state.map.elements_row_major_iter().for_each(|element| {
            let color = match element.get_tile_type() {
                TileType::UNKNOWN => Color::GRAY,
                TileType::EMPTY => Color::OLIVE,
                TileType::TENT => Color::YELLOW,
                TileType::TREE => Color::GREEN,
                _ => Color::RED,
            };
            draw.rect(element.position.get().into(), element.get_tile_size()).color(color);
        });

        gfx.render(&draw);


        let ui = GameUI::draw_ui(state);
        let ui = plugins.egui(ui);
        gfx.render(&ui);
    }
}

impl Game {
    fn process_input(app: &mut App, state: &mut State) {
        if app.mouse.left_was_pressed() {
            if let Some(index) = state.map.as_row_major().iter().position(|e| {
                e.is_in_boundary_box(app.mouse.position().into())
            }) {
                if let Some(tile) = state.map.get_mut_row_major(index) {
                    tile.switch_to_next_tile_type();
                };
            };
        };
    }

    fn calculate_tile_pos(app: &mut App, state: &mut State) {
        const PADDING: f32 = 10f32;
        const OFFSET: f32 = PADDING / 2f32;

        let map_row_len = &state.map.row_len();
        let map_col_len = &state.map.column_len();

        let window_size = &app.backend.window().size();
        let window_center = (window_size.0 / 2, window_size.1 / 2);

        let tile_with_padding: (f32, f32) = (state.get_tile_size().0 + PADDING,
                                             state.get_tile_size().1 + PADDING);

        let center_of_map_in_x = (map_row_len / 2) as f32;
        let center_of_map_in_y = (map_col_len / 2) as f32;

        let relative_offset = (
            window_center.0 as f32 - (tile_with_padding.0 * center_of_map_in_x),
            window_center.1 as f32 - (tile_with_padding.1 * center_of_map_in_y)
        );

        let off_set_x = relative_offset.0 + OFFSET;
        let off_set_y = relative_offset.1 + OFFSET;

        for (row_idx, row) in state.map.rows_iter().enumerate() {
            for (col_idx, element) in row.enumerate() {
                let position = (
                    (tile_with_padding.0 * col_idx as f32) + off_set_x,
                    (tile_with_padding.1 * row_idx as f32) + off_set_y,
                );

                element.position.set(position.into());
            }
        }
    }
}