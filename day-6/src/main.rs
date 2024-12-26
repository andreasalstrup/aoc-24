use std::fs::File;
use std::io::{Read, Result};
use std::str::FromStr;
use std::{isize, usize};

use Direction::*;

#[derive(PartialEq, Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_u8(b: u8) -> Option<Self> {
        match b {
            b'^' => Some(Direction::Up),
            b'v' => Some(Direction::Down),
            b'<' => Some(Direction::Left),
            b'>' => Some(Direction::Right),
            _ => None,
        }
    }

    fn offset(&self) -> (isize, isize) {
        match self {
            Up => (-1, 0),
            Down => (1, 0),
            Left => (0, -1),
            Right => (0, 1),
        }
    }
}

impl From<Direction> for u8 {
    fn from(value: Direction) -> Self {
        match value {
            Up => b'^',
            Down => b'v',
            Left => b'<',
            Right => b'>',
        }
    }
}

impl Iterator for Direction {
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        let directions = [Up, Right, Down, Left];
        let index = directions.iter().position(|&d| d == *self)?;
        Some(directions[(index + 1) % directions.len()])
    }
}

#[derive(Debug)]
struct Grid {
    data: Vec<Vec<u8>>,
    rows: usize,
    cols: usize,
    guard_pos: ((usize, usize), Direction),
}

impl ToString for Grid {
    fn to_string(&self) -> String {
        self.data
            .iter()
            .map(|row| row.iter().map(|&c| c as char).collect())
            .collect::<Vec<String>>()
            .join("\n")
    }
}

#[derive(Debug)]
struct ParseGridError;

impl FromStr for Grid {
    type Err = ParseGridError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let data: Vec<Vec<u8>> = s
            .lines()
            .map(|line| line.chars().map(|c| c as u8).collect())
            .collect();
        let rows = data.len();
        let cols = data.first().map(|r| r.len()).ok_or(ParseGridError)?;
        let guard_pos = (0..rows)
            .flat_map(|row| (0..cols).map(move |col| (row, col)))
            .find_map(|(row, col)| Direction::from_u8(data[row][col]).map(|dir| ((row, col), dir)))
            .ok_or(ParseGridError)?;
        Ok(Grid {
            data,
            rows,
            cols,
            guard_pos,
        })
    }
}

impl Iterator for Grid {
    type Item = ((usize, usize), Direction);

    fn next(&mut self) -> Option<Self::Item> {
        let (mut row, mut col) = (self.guard_pos.0 .0 as isize, self.guard_pos.0 .1 as isize);
        let (row_off, col_off) = self.guard_pos.1.offset();
        let mut guard_pos = self.guard_pos.1;

        let mut prev_pos = (row as isize - row_off, col as isize - col_off);

        while self.get(row as isize, col as isize) != b'#' {
            let current_val = self.get_mut(row, col)?;
            *current_val = b'X';

            (row, col) = ((row as isize + row_off), (col as isize + col_off));
            prev_pos = (row as isize - row_off, col as isize - col_off);

            if let Some(_) = self.get_mut(row, col) {
                continue;
            } else {
                return None;
            }
        }

        let prev_val = self.get_mut(prev_pos.0, prev_pos.1)?;
        *prev_val = guard_pos.into();

        let new_guard_pos = (
            (prev_pos.0 as usize, prev_pos.1 as usize),
            guard_pos.next()?,
        );
        self.guard_pos = new_guard_pos;

        Some(self.guard_pos)
    }
}

impl Grid {
    pub fn get(&self, row: isize, col: isize) -> u8 {
        *self
            .data
            .get(row as usize)
            .and_then(|r| r.get(col as usize))
            .unwrap_or(&b'.')
    }

    pub fn get_mut(&mut self, row: isize, col: isize) -> Option<&mut u8> {
        self.data
            .get_mut(row as usize)
            .and_then(|r| r.get_mut(col as usize))
    }

    pub fn part_one(&mut self) -> u32 {
        for _ in self.by_ref() {}
        (0..self.rows)
            .flat_map(|row| (0..self.cols).map(move |col| (row, col)))
            .map(|(row, col)| self.data[row][col])
            .fold(0, |acc, c| if c == b'X' { acc + 1 } else { acc })
    }
}

fn main() -> Result<()> {
    let mut file = File::open("./day-6/input.txt")?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    let mut grid = Grid::from_str(&buffer).unwrap();
    println!("Part one: {}", grid.part_one());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."
    }

    #[test]
    fn part_one() {
        let mut grid = Grid::from_str(input()).unwrap();
        assert_eq!(41, grid.part_one());
    }
}
