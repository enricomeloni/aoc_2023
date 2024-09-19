use std::num::NonZeroU8;

use itertools::Itertools;
use utils::{get_input_path, read_lines};

#[derive(PartialEq, Eq, Clone, Copy)]
enum Tile {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start,
}

const TILE_TO_CHAR: [char; 8] = [
    '|', // Vertical
    '-', // Horizontal
    'L', // NorthEast
    'J', // NorthWest
    '7', // SouthWest
    'F', // SouthEast
    '.', // Ground
    'S', // Start
];


fn map_char_to_tile(ch: char) -> Tile {
    match ch {
        '|' => Tile::Vertical,
        '-' => Tile::Horizontal,
        'L' => Tile::NorthEast,
        'J' => Tile::NorthWest,
        '7' => Tile::SouthWest,
        'F' => Tile::SouthEast,
        'S' => Tile::Start,
        '.' => Tile::Ground,
        _ => panic!("Should not happen"),
    }
}

fn map_tile_to_char(tile: Tile) -> char {
    TILE_TO_CHAR[tile as usize]
}



enum Direction {
    North,
    South,
    West,
    East,
}

type Offset = (i32, i32);

fn map_direction_to_offset(direction: Direction) -> Offset {
    match direction {
        Direction::North => (-1, 0),
        Direction::South => (1, 0),
        Direction::East => (0, 1),
        Direction::West => (0, -1),
    }
}

struct Adjancents {
    north: Option<Tile>,
    south: Option<Tile>,
    east: Option<Tile>,
    west: Option<Tile>,
}

type Pos = (usize, usize);

pub trait MoveInDirectionExt {
    fn move_in_direction(&self, direction: Direction) -> Option<Pos>;
}

impl MoveInDirectionExt for Pos {
    fn move_in_direction(&self, direction: Direction) -> Option<Pos> {
        let offset = map_direction_to_offset(direction);
        if self.0 as i32 > -offset.0 && self.1 as i32 > -offset.1 {
            return Some((
                (self.0 as i32 + offset.0) as usize,
                (self.1 as i32 + offset.1) as usize,
            ));
        }
        return None;
    }
}

struct Map {
    tiles: Vec<Vec<Tile>>,
    start_pos: Pos,
}

impl Map {
    fn get_tile(&self, pos: Pos) -> &Tile {
        return &self.tiles[pos.0][pos.1];
    }

    fn get_tile_in_direction(&self, pos: Pos, direction: Direction) -> Option<Tile> {
        Some(*self.get_tile(pos.move_in_direction(direction)?))
    }

    fn get_adjancents(&self, pos: Pos) -> Adjancents {
        Adjancents {
            north: self.get_tile_in_direction(pos, Direction::North),
            east: self.get_tile_in_direction(pos, Direction::East),
            west: self.get_tile_in_direction(pos, Direction::West),
            south: self.get_tile_in_direction(pos, Direction::South),
        }
    }

    fn get_max_distance(&self) -> u32 {
        todo!()
    }
}

fn find_start_pos(tiles: &Vec<Vec<Tile>>) -> Option<Pos> {
    for (line_idx, tile_line) in tiles.iter().enumerate() {
        for (tile_idx, tile) in tile_line.iter().enumerate() {
            if *tile == Tile::Start {
                return Some((line_idx, tile_idx));
            }
        }
    }

    return None;
}

fn main() {
    let input_path = get_input_path(10, Some("example1.txt"));

    if let Ok(lines) = read_lines(&input_path) {
        let lines = lines.map(|line| line.unwrap()).collect_vec();

        let tiles = lines
            .iter()
            .map(|line| line.chars().map(map_char_to_tile).collect_vec())
            .collect_vec();

        let start_pos = find_start_pos(&tiles).unwrap();

        let map = Map { tiles, start_pos };

        println!("Finish");
    }
}
