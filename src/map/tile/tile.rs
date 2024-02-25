use std::cell::Cell;
use enum_map::Enum;
use crate::{
    map::{
        tile::{
            grid_position::*,
            point::Point,
        }
    }
};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Enum)]
pub enum TileType {
    UNKNOWN,
    EMPTY,
    TENT,
    TREE,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Tile {
    tile_type: TileType,
    coord: GridPosition,
    pub position: Cell<Point>,
    size: (f32, f32),
}

impl Default for Tile {
    fn default() -> Self {
        Tile {
            tile_type: TileType::UNKNOWN,
            coord: GridPosition::default(),
            position: Cell::new(Point::default()),
            size: (0f32, 0f32),
        }
    }
}

impl Tile {
    pub fn new(tile_type: TileType, coord: GridPosition, position: Point, size: (f32, f32)) -> Self {
        Tile {
            tile_type,
            coord,
            position: Cell::new(position),
            size,
        }
    }

    pub fn get_tile_type(&self) -> &TileType {
        &self.tile_type
    }

    pub fn get_tile_size(&self) -> (f32, f32) {
        self.size
    }

    pub fn switch_to_next_tile_type(&mut self) {
        self.tile_type = match self.tile_type {
            TileType::UNKNOWN => TileType::EMPTY,
            TileType::EMPTY => TileType::TENT,
            TileType::TENT => TileType::UNKNOWN,
            TileType::TREE => TileType::TREE,
        };
    }

    pub fn is_in_boundary_box(&self, position: Point) -> bool {
        let top_left_pos = self.position.get();
        let bottom_right_pos = top_left_pos + self.size;

        let is_in_x = position.0 >= top_left_pos.0 && position.0 <= bottom_right_pos.0;
        let is_in_y = position.1 >= top_left_pos.1 && position.1 <= bottom_right_pos.1;

        if is_in_x && is_in_y {
            true
        } else {
            false
        }
    }
}

