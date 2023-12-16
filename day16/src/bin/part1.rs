use std::collections::HashSet;

use itertools::*;

#[derive(Clone, Copy)]
struct Tile {
    visit: bool,
    ty: TileType,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        Self {
            visit: false,
            ty: value.into(),
        }
    }
}

#[derive(Clone, Copy)]
enum TileType {
    Empty,
    MirrorF,
    MirrorB,
    SplitterH,
    SplitterV,
}

impl From<char> for TileType {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Empty,
            '/' => Self::MirrorF,
            '\\' => Self::MirrorB,
            '-' => Self::SplitterH,
            '|' => Self::SplitterV,
            _ => panic!(),
        }
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Beam {
    dir: Direction,
    pos: (i32, i32),
}

impl Beam {
    fn from(dir: Direction, pos: (i32, i32)) -> Self {
        Self { dir, pos }
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    Top,
    Bottom,
    Left,
    Right,
}

struct Field {
    field: Vec<Vec<Tile>>,
    width: i32,
    height: i32,
}

impl Field {
    fn from(field: Vec<Vec<Tile>>) -> Self {
        Field {
            width: field[0].len() as i32,
            height: field.len() as i32,
            field,
        }
    }
    fn count(mut self) -> usize {
        let mut beams = vec![Beam::from(Direction::Right, (0, -1))];

        let mut poses: HashSet<Beam> = HashSet::new();

        while !beams.is_empty() {
            beams = beams
                .into_iter()
                .filter(|beam| poses.insert(*beam))
                .collect_vec();
            beams = beams
                .into_iter()
                .flat_map(|Beam { dir, mut pos }| {
                    use Direction as Dir;
                    match dir {
                        Dir::Top => pos.0 -= 1,
                        Dir::Bottom => pos.0 += 1,
                        Dir::Left => pos.1 -= 1,
                        Dir::Right => pos.1 += 1,
                    }
                    if (0..self.height).contains(&pos.0) && (0..self.width).contains(&pos.1) {
                        self.field[pos.0 as usize][pos.1 as usize].visit = true;
                        match (dir, self.field[pos.0 as usize][pos.1 as usize].ty) {
                            (Dir::Top, TileType::MirrorF) => vec![Beam {
                                dir: Dir::Right,
                                pos,
                            }],
                            (Dir::Bottom, TileType::MirrorF) => vec![Beam {
                                dir: Dir::Left,
                                pos,
                            }],
                            (Dir::Right, TileType::MirrorF) => vec![Beam { dir: Dir::Top, pos }],
                            (Dir::Left, TileType::MirrorF) => vec![Beam {
                                dir: Dir::Bottom,
                                pos,
                            }],
                            (Dir::Top, TileType::MirrorB) => vec![Beam {
                                dir: Dir::Left,
                                pos,
                            }],
                            (Dir::Bottom, TileType::MirrorB) => vec![Beam {
                                dir: Dir::Right,
                                pos,
                            }],
                            (Dir::Right, TileType::MirrorB) => vec![Beam {
                                dir: Dir::Bottom,
                                pos,
                            }],
                            (Dir::Left, TileType::MirrorB) => vec![Beam { dir: Dir::Top, pos }],
                            (Dir::Left | Dir::Right, TileType::SplitterV) => {
                                vec![
                                    Beam {
                                        dir: Dir::Bottom,
                                        pos,
                                    },
                                    Beam { dir: Dir::Top, pos },
                                ]
                            }
                            (Dir::Top | Dir::Bottom, TileType::SplitterH) => {
                                vec![
                                    Beam {
                                        dir: Dir::Left,
                                        pos,
                                    },
                                    Beam {
                                        dir: Dir::Right,
                                        pos,
                                    },
                                ]
                            }
                            (_, _) => vec![Beam { dir, pos }],
                        }
                    } else {
                        vec![]
                    }
                })
                .collect_vec();
        }

        self.field
            .iter()
            .flatten()
            .filter(|tile| tile.visit)
            .count()
    }
}

fn solution(input: &str) -> usize {
    let field = Field::from(
        input
            .lines()
            .map(|line| line.chars().map(Tile::from).collect_vec())
            .collect_vec(),
    );

    field.count()
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
        let input = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";
        let res = solution(input);

        assert_eq!(res, 46);
    }
}
