use std::{
    collections::HashMap,
    convert::{TryFrom, TryInto},
    fmt::Debug,
};

use itertools::Itertools;

#[derive(PartialEq, Copy, Clone)]
enum SpaceType {
    Floor,
    Seat(bool),
}
impl Eq for SpaceType {}

impl TryFrom<char> for SpaceType {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(SpaceType::Floor),
            'L' => Ok(SpaceType::Seat(false)),
            '#' => Ok(SpaceType::Seat(true)),
            _ => Err(()),
        }
    }
}

type Position = ((usize, usize), SpaceType);

#[derive(PartialEq, Clone)]
pub struct Area {
    cols: usize,
    rows: usize,
    places: Vec<SpaceType>,
}

impl Debug for Area {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.places.chunks_exact(self.cols) {
            for s in row {
                match s {
                    SpaceType::Floor => f.write_str(".")?,
                    SpaceType::Seat(false) => f.write_str("L")?,
                    SpaceType::Seat(true) => f.write_str("#")?,
                }
            }
            f.write_str("\n")?
        }
        Ok(())
    }
}

impl Area {
    fn get_pos(&self, row: usize, col: usize) -> &SpaceType {
        let idx = row * self.cols + col;
        self.places.get(idx).expect(&format!(
            "out of range {}of{} {}of{}",
            row, self.rows, col, self.cols
        ))
    }
    fn get_neighbors(&self, row: usize, col: usize) -> Vec<&SpaceType> {
        let min_row = row.saturating_sub(1);
        let max_row = (row + 1).min(self.rows - 1);

        let min_col = col.saturating_sub(1);
        let max_col = (col + 1).min(self.cols - 1);

        let mut result = Vec::new();
        for r in min_row..=max_row {
            for c in min_col..=max_col {
                // println!("checking {} {}", r, c);
                if r != row || c != col {
                    result.push(self.get_pos(r, c));
                }
            }
        }
        result
    }
    fn get_occ_neighbors(&self, row: usize, col: usize) -> usize {
        self.get_neighbors(row, col)
            .into_iter()
            .filter(|s| **s == SpaceType::Seat(true))
            .count()
    }

    fn step(&self) -> Area {
        let spaces = self
            .get_places()
            .map(|((r, c), p)| match p {
                SpaceType::Floor => SpaceType::Floor,
                SpaceType::Seat(false) => SpaceType::Seat(self.get_occ_neighbors(r, c) == 0),
                SpaceType::Seat(true) => SpaceType::Seat(self.get_occ_neighbors(r, c) < 4),
            })
            .collect_vec();
        Area {
            cols: self.cols,
            rows: self.rows,
            places: spaces,
        }
    }

    fn index_to_pos(&self, index: usize) -> (usize, usize) {
        (index / self.cols, index % self.cols)
    }

    fn get_places(&self) -> impl Iterator<Item = Position> + '_ {
        self.places
            .iter()
            .enumerate()
            .map(move |(i, p)| (self.index_to_pos(i), *p))
    }

    fn get_occupied_count(&self) -> usize {
        self.places
            .iter()
            .filter(|p| **p == SpaceType::Seat(true))
            .count()
    }
}

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Area {
    let line_length = input.lines().next().unwrap().len();
    debug_assert!(input.lines().map(|line| line.len()).all_equal());
    let places = input
        .lines()
        .flat_map(|l| l.chars().map(|c| c.try_into().unwrap()))
        .collect_vec();
    Area {
        cols: line_length,
        rows: places.len() / line_length,
        places,
    }
}

#[aoc(day11, part1)]
pub fn part1(input: &Area) -> usize {
    // println!("{:?}", input);
    let mut prev = input.step();
    // println!("{:?}", prev);
    loop {
        let next = prev.step();
        // println!("{:?}", next);
        if next == prev {
            return next.get_occupied_count();
        } else {
            prev = next;
        }
    }
}

#[aoc(day11, part2)]
pub fn part2(input: &Area) -> u128 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            37,
            part1(&input_generator(
                "\
L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL"
            ))
        );
    }
    #[test]
    fn part1_mini() {
        assert_eq!(
            4,
            input_generator(
                "\
#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##"
            )
            .get_occ_neighbors(0, 3)
        );
    }
}
