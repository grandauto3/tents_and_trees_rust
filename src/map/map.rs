use std::collections::HashSet;
use array2d::Array2D;
use rand::Rng;

use crate::{
    map::{
        tile::{
            tile::*,
            grid_position::*,
            point::Point
        }
    }
};

pub fn create_map((size_x, size_y): (usize, usize)) -> Array2D<Tile> {
    let tree_set = get_random_tree_pos((size_x, size_y));

    // start at -1, so we can safely increment it in the closure to 0
    // because the if-clause will be called on the first iteration even though the row is not completed
    let mut row_counter = -1;
    let mut column_counter = 0;
    let incrementor = || {
        let tmp_column = column_counter;
        let column = tmp_column % size_y;
        column_counter += 1;

        if column == 0 {
            row_counter += 1;
        }
        let tmp_row = row_counter as usize;
        let row = tmp_row % size_x;

        let grid_pos = GridPosition::new((row, column));

        let tile_type = if tree_set.contains(&grid_pos) {
            TileType::TREE
        } else {
            TileType::UNKNOWN
        };

        Tile::new(tile_type, grid_pos, Point::default())
    };
    Array2D::filled_by_row_major(incrementor, size_x, size_y)
}

pub fn get_neighbours((row, column): (usize, usize), map: &Array2D<Tile>) -> Vec<Option<&Tile>> {
    let mut result: Vec<Option<&Tile>> = vec![];
    if let Some(_) = map.get(column, row) {
        for row_inner in -1..=1 {
            for column_inner in -1..=1 {
                if column_inner == 0 && row_inner == 0 {
                    continue;
                }
                let checked_column = column.checked_add_signed(column_inner);
                let checked_row = row.checked_add_signed(row_inner);

                if checked_column.is_some() && checked_row.is_some() {
                    result.push(map.get(checked_row.unwrap(), checked_column.unwrap()));
                } else { result.push(None) }
            }
        }
    }
    result
}

pub fn draw_map(map: &Array2D<Tile>) {
    let border_size = map.row_len() + 2;

    let line: String = (0..border_size).map(move |_| { "-" }).collect();

    println!("{}", line);
    for row_iter in map.rows_iter() {
        print!("|");
        for e in row_iter {
            match e.get_tile_type() {
                TileType::UNKNOWN => print!("0"),
                TileType::EMPTY => print!("x"),
                TileType::TENT => print!("T"),
                TileType::TREE => print!("t"),
            }
        }
        println!("|");
    }
    println!("{}", line);
}

fn get_random_tree_pos((size_x, size_y): (usize, usize)) -> HashSet<GridPosition> {
    let mut tree_set = HashSet::new();

    let mut rng = rand::thread_rng();
    for _ in 0..10 {
        //loop to prevent skipping duplicated values
        'inner: loop {
            let rnd_row = rng.gen_range(0..size_x);
            let rnd_column = rng.gen_range(0..size_y);
            let did_insert = tree_set.insert(GridPosition::new((rnd_row, rnd_column)));

            if did_insert {
                break 'inner;
            }
        }
    }
    tree_set
}
