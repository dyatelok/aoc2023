#![feature(iter_intersperse)]
use std::fmt;

#[derive(PartialEq, Eq, Clone, Copy)]
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

    roll(&mut field);

    weight(&field)
}

fn roll(field: &mut Vec<Vec<Tile>>) {
    for _ in 0..field.len() {
        for i in 0..field.len() - 1 {
            roll_row(field, i);
        }
    }
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
    use crate::debug;
    use crate::roll;
    use crate::transform;
    use crate::weight;

    #[test]
    fn test_roll() {
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
        let target = "OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#....";
        let mut field = transform(field);
        let target = transform(target);

        debug(&field);
        println!();

        roll(&mut field);

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
}
