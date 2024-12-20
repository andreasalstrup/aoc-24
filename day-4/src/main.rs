use std::fs::File;
use std::io::{prelude::*, Result};
use std::{isize, usize};

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
        let cols = data.first().map(|v| v.len()).unwrap_or(0);
        Grid { data, rows, cols }
    }

    pub fn get(&self, row: isize, col: isize) -> u8 {
        *self
            .data
            .get(row as usize)
            .and_then(|r| r.get(col as usize))
            .unwrap_or(&b'.')
    }

    pub fn xmas_count(&self, row: usize, col: usize) -> usize {
        [
            Up,
            Down,
            Left,
            Right,
            VerticalTopLeft,
            VerticalTopRight,
            VerticalBottomLeft,
            VerticalBottomRight,
        ]
        .iter()
        .filter(|direction| {
            (0..4).all(|i| {
                let (offset_row, offset_col) = direction.offset();
                let new_row = row as isize + (offset_row * i);
                let new_col = col as isize + (offset_col * i);
                self.get(new_row, new_col) == b"XMAS"[i as usize]
            })
        })
        .count()
    }

    pub fn mas_count(&self, row: isize, col: isize) -> bool {
        let cross =
            |top_row: isize, top_col: isize, bottom_row: isize, bottom_col: isize| -> [u8; 2] {
                [self.get(top_row, top_col), self.get(bottom_row, bottom_col)]
            };

        let (top_left_row, top_left_col) = VerticalTopLeft.offset();
        let (bottom_right_row, bottom_right_col) = VerticalBottomRight.offset();
        let cross_1 = cross(
            row + top_left_row,
            col + top_left_col,
            row + bottom_right_row,
            col + bottom_right_col,
        );

        let (top_right_row, top_right_col) = VerticalTopRight.offset();
        let (bottom_left_row, bottom_left_col) = VerticalBottomLeft.offset();
        let cross_2 = cross(
            row + top_right_row,
            col + top_right_col,
            row + bottom_left_row,
            col + bottom_left_col,
        );

        [cross_1, cross_2]
            .iter()
            .all(|r#match| r#match == b"MS" || r#match == b"SM")
    }
}

fn part_one(grid: &Grid) -> usize {
    (0..grid.rows)
        .flat_map(|row| (0..grid.cols).map(move |col| (row, col)))
        .filter(|&(row, col)| grid.data[row][col] == b'X')
        .fold(0, |acc, (row, col)| acc + grid.xmas_count(row, col))
}

fn part_two(grid: &Grid) -> usize {
    (0..grid.rows)
        .flat_map(|row| (0..grid.cols).map(move |col| (row, col)))
        .filter(|&(row, col)| grid.data[row][col] == b'A')
        .fold(0, |acc, (row, col)| {
            if grid.mas_count(row as isize, col as isize) {
                acc + 1
            } else {
                acc
            }
        })
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

    let grid = &Grid::new(&buffer);

    println!("XMAS cout: {}", part_one(grid));
    println!("MAS cout: {}", part_two(grid));

    Ok(())
}
