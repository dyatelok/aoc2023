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

    let mut ctr = 0;

    loop {
        ctr += diff(&field[ptr1], &field[ptr2]);
        if ptr1 == 0 || ptr2 == field.len() - 1 || ctr > 1 {
            break;
        }
        ptr1 -= 1;
        ptr2 += 1;
    }

    ctr == 1
}

fn diff(row1: &Vec<Tile>, row2: &Vec<Tile>) -> usize {
    let mut ctr = 0;

    for (tile1, tile2) in row1.into_iter().zip(row2.into_iter()) {
        if tile1 != tile2 {
            ctr += 1;
        }
    }

    ctr
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
    use crate::solution;

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
        assert_eq!(res, 400);
    }
}
