#![feature(iter_intersperse)]
use std::fmt;

#[derive(PartialEq, Eq, Clone, Copy)]
enum Tile {
    Ash,
    Rock,
}

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Ash => '.',
                Tile::Rock => '#',
            }
        )
    }
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Tile::Ash,
            '#' => Tile::Rock,
            _ => panic!("{value}"),
        }
    }
}

fn solution(input: &str) -> usize {
    input.split("\n\n").map(handle_field).sum()
}

fn handle_field(field: &str) -> usize {
    let field = transform(field);

    let mut res = 0;

    res += reflections(&field) * 100;

    let field = transpose(field);

    res += reflections(&field);

    res
}

fn transform(field: &str) -> Vec<Vec<Tile>> {
    field
        .lines()
        .map(|line| line.chars().map(Tile::from).collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

// use itertools::*;

#[allow(clippy::ptr_arg)]
fn reflections(field: &Vec<Vec<Tile>>) -> usize {
    for i in 0..field.len() - 1 {
        if reflects(field, i) {
            return i + 1;
        }
    }
    0
}

#[allow(clippy::ptr_arg)]
fn reflects(field: &Vec<Vec<Tile>>, axis: usize) -> bool {
    let mut ptr1 = axis;
    let mut ptr2 = axis + 1;

    loop {
        if field[ptr1] != field[ptr2] {
            return false;
        }
        if ptr1 == 0 || ptr2 == field.len() - 1 {
            break;
        }
        ptr1 -= 1;
        ptr2 += 1;
    }

    true
}

fn transpose<T>(orig: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let mut transposed = (0..orig[0].len())
        .map(|_| Vec::with_capacity(orig.len()))
        .collect::<Vec<_>>();

    for orow in orig {
        for (item, trow) in orow.into_iter().zip(&mut transposed) {
            trow.push(item);
        }
    }

    transposed
}

fn main() {
    let input = include_str!("../../input1.txt");
    let res = solution(input);
    println!("{res}");
}

#[cfg(test)]
mod tests {
    use crate::{reflections, reflects, solution, transform, transpose};

    #[test]
    fn test() {
        let field = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        let res = solution(field);
        assert_eq!(res, 405);
    }
    #[test]
    fn reflects1() {
        let field = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.";
        let field = transform(field);
        let field = transpose(field);

        assert!(reflects(&field, 4));
    }
    #[test]
    fn reflects2() {
        let field = "#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        let field = transform(field);

        assert!(reflects(&field, 3));
    }
    #[test]
    fn reflections1() {
        let field = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.";
        let field = transform(field);
        let field = transpose(field);

        assert_eq!(reflections(&field), 5);
    }
    #[test]
    fn reflections2() {
        let field = "#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        let field = transform(field);

        assert_eq!(reflections(&field), 4);
    }
}
