use std::cell::Cell;
use crate::{
    map::{
        tile::{
            grid_position::*,
            point::Point,
        }
    }
};

pub const SIZE_OF_TILE: (f32, f32) = (50f32, 50f32);

#[derive(Clone, Debug, PartialEq)]
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
            size: SIZE_OF_TILE,
        }
    }
}

impl Tile {
    pub fn new(tile_type: TileType, coord: GridPosition, position: Point) -> Self {
        Tile {
            tile_type,
            coord,
            position: Cell::new(position),
            ..Self::default()
        }
    }

    pub fn get_tile_type(&self) -> &TileType {
        &self.tile_type
    }

    pub fn get_tile_size(&self) -> (f32,f32) {
        self.size
    }
}

