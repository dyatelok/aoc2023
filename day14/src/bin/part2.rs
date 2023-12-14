#![feature(iter_intersperse)]
use std::{collections::HashMap, fmt};

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
enum Tile {
    Empty,
    Cube,
    Rounded,
}

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Empty => '.',
                Tile::Cube => '#',
                Tile::Rounded => 'O',
            }
        )
    }
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Tile::Empty,
            '#' => Tile::Cube,
            'O' => Tile::Rounded,
            _ => panic!("{value}"),
        }
    }
}

#[allow(clippy::ptr_arg, unused)]
fn debug(field: &Vec<Vec<Tile>>) {
    for row in field {
        println!("{row:?}");
    }
}

fn solution(input: &str) -> usize {
    let mut field = transform(input);

    let mut states = HashMap::new();

    const ITERATIONS: usize = 1000000000;

    for i in 0..ITERATIONS {
        field = cycle(field);

        if let Some(id) = states.insert(field.clone(), i) {
            let cycle_len = i - id;

            let cycle_pos = (ITERATIONS - i) % cycle_len + cycle_len - 1;

            for _ in 0..cycle_pos {
                field = cycle(field);
            }

            return weight(&field);
        }
    }

    weight(&field)
}

#[allow(clippy::ptr_arg)]
fn cycle(mut field: Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
    for _ in 0..4 {
        field = roll(field);
        field = rotate(field);
    }

    field
}

fn rotate(original: Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
    let mut rotated = (0..original[0].len()).map(|_| vec![]).collect::<Vec<_>>();

    for original_row in original {
        for (item, rotated_row) in original_row.into_iter().zip(&mut rotated) {
            rotated_row.push(item);
        }
    }

    for rotated_row in rotated.iter_mut() {
        rotated_row.reverse();
    }

    rotated
}

fn roll(mut field: Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
    for _ in 0..field.len() {
        for i in 0..field.len() - 1 {
            roll_row(&mut field, i);
        }
    }

    field
}

#[allow(clippy::ptr_arg)]
fn roll_row(field: &mut Vec<Vec<Tile>>, i: usize) {
    let (part1, part2) = field.split_at_mut(i + 1);
    let row1 = part1.last_mut().unwrap();
    let row2 = part2.first_mut().unwrap();

    for (elem1, elem2) in row1.iter_mut().zip(row2.iter_mut()) {
        if *elem1 == Tile::Empty && *elem2 == Tile::Rounded {
            std::mem::swap(elem1, elem2);
        }
    }
}

#[allow(clippy::ptr_arg)]
fn weight(field: &Vec<Vec<Tile>>) -> usize {
    field
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter().filter(|item| **item == Tile::Rounded).count() * (field.len() - i)
        })
        .sum()
}

fn transform(field: &str) -> Vec<Vec<Tile>> {
    field
        .lines()
        .map(|line| line.chars().map(Tile::from).collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn main() {
    let input = include_str!("../../input1.txt");
    let res = solution(input);
    println!("{res}");
}

#[cfg(test)]
mod tests {
    use crate::cycle;
    use crate::debug;
    use crate::solution;
    use crate::transform;
    use crate::weight;

    #[test]
    fn test_cycle() {
        let field = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        let target = ".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....";

        let mut field = transform(field);
        let target = transform(target);

        debug(&field);
        println!();

        field = cycle(field);

        debug(&field);
        println!();
        debug(&target);

        assert_eq!(field, target);
    }
    #[test]
    fn test_load() {
        let field = "OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#....";
        let field = transform(field);

        assert_eq!(weight(&field), 136);
    }
    #[test]
    fn test_solution() {
        let field = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        let res = solution(field);

        assert_eq!(res, 64);
    }
}
