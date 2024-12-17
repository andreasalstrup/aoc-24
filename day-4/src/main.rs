use std::fs::File;
use std::io::{prelude::*, Result};
use std::usize;

use Direction::*;

enum Direction {
    Up,
    Down,
    Left,
    Right,
    VerticalTopLeft,
    VerticalTopRight,
    VerticalBottomLeft,
    VerticalBottomRight,
}

impl Direction {
    fn offset(&self) -> (isize, isize) {
        match self {
            Up => (-1, 0),
            Down => (1, 0),
            Left => (0, -1),
            Right => (0, 1),
            VerticalTopLeft => (-1, -1),
            VerticalTopRight => (-1, 1),
            VerticalBottomLeft => (1, -1),
            VerticalBottomRight => (1, 1),
        }
    }
}

#[derive(Debug)]
struct Grid {
    data: Vec<Vec<u8>>,
    rows: usize,
    cols: usize,
}

impl Grid {
    fn new(data: &str) -> Grid {
        let data: Vec<Vec<u8>> = data
            .lines()
            .map(|line| line.chars().map(|c| c as u8).collect())
            .collect();
        let rows = data.len();
        let cols = data[0].len();
        Grid { data, rows, cols }
    }

    pub fn xmas_count(&self, row: usize, col: usize) -> usize {
        // use Direction::*;

        let directions = [
            Up,
            Down,
            Left,
            Right,
            VerticalTopLeft,
            VerticalTopRight,
            VerticalBottomLeft,
            VerticalBottomRight,
        ];

        directions
            .iter()
            .filter(|direction| {
                (0..4).all(|i| {
                    let (offset_row, offset_col) = direction.offset();
                    let new_row = row as isize + (offset_row * i);
                    let new_col = col as isize + (offset_col * i);
                    *self
                        .data
                        .get(new_row as usize)
                        .and_then(|r| r.get(new_col as usize))
                        .unwrap_or(&b'.')
                        == b"XMAS"[i as usize]
                })
            })
            .count()
    }
}

fn part_one(grid: Grid) -> usize {
    (0..grid.rows)
        .flat_map(|row| (0..grid.cols).map(move |col| (row, col)))
        .filter(|&(row, col)| grid.data[row][col] == b'X')
        .fold(0, |acc, (row, col)| acc + grid.xmas_count(row, col))
}

fn main() -> Result<()> {
    let mut file = File::open("./day-4/input.txt")?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    let test = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    let grid = Grid::new(&buffer);

    // 2517
    println!("XMAS cout: {}", part_one(grid));

    Ok(())
}
