use itertools::*;

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
enum Dir {
    L,
    R,
    U,
    D,
}

impl From<&str> for Dir {
    fn from(value: &str) -> Self {
        match value {
            "L" => Dir::L,
            "R" => Dir::R,
            "U" => Dir::U,
            "D" => Dir::D,
            _ => panic!("{value}"),
        }
    }
}

#[derive(Clone, Copy)]
struct Move {
    len: i64,
    dir: Dir,
}

impl Move {
    fn from(len: i64, dir: Dir) -> Self {
        Self { len, dir }
    }
}

#[derive(Clone, Copy)]
struct Pos {
    x: i64,
    y: i64,
}

impl Pos {
    fn from(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

use std::ops::AddAssign;
impl AddAssign<Move> for Pos {
    fn add_assign(&mut self, rhs: Move) {
        match rhs.dir {
            Dir::L => self.x -= rhs.len,
            Dir::R => self.x += rhs.len,
            Dir::U => self.y -= rhs.len,
            Dir::D => self.y += rhs.len,
        }
    }
}

fn solution(input: &str) -> f32 {
    let mut pos = Pos::from(0, 0);
    let mut perimeter = 0;

    let positions = input
        .lines()
        .map(|line| {
            let mut splited = line.split_whitespace();
            let dir = Dir::from(splited.next().unwrap().trim());
            let len = splited.next().unwrap().parse::<i64>().unwrap();
            perimeter += len;
            Move::from(len, dir)
        })
        .map(|mov| {
            pos += mov;
            pos
        });

    positions
        .tuple_windows()
        .map(|(c1, c2)| c1.x * c2.y - c1.y * c2.x)
        .sum::<i64>() as f32
        / 2.0
        + perimeter as f32 / 2.0
        + 1.0
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
        let input = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
        let res = solution(input);

        assert_eq!(res, 62.0);
    }
}
