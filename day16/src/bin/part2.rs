use std::collections::HashSet;

use itertools::*;

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

struct Field<'a> {
    field: &'a Vec<Vec<TileType>>,
    visits: Vec<Vec<bool>>,
    width: i32,
    height: i32,
}

impl<'a> Field<'a> {
    fn from(field: &'a Vec<Vec<TileType>>) -> Self {
        let width = field[0].len();
        let height = field.len();
        Field {
            field,
            visits: vec![vec![false; width]; height],
            width: width as i32,
            height: height as i32,
        }
    }
    fn count(mut self, start: Beam) -> usize {
        let mut beams = vec![start];

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
                        self.visits[pos.0 as usize][pos.1 as usize] = true;
                        match (dir, self.field[pos.0 as usize][pos.1 as usize]) {
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

        self.visits.iter().flatten().filter(|visit| **visit).count()
    }
}

fn solution(input: &str) -> usize {
    let types = input
        .lines()
        .map(|line| line.chars().map(TileType::from).collect_vec())
        .collect_vec();

    let width = types[0].len() as i32;
    let height = types.len() as i32;

    let mut brightness = 0;

    for y in 0..height {
        brightness = brightness.max(Field::from(&types).count(Beam {
            dir: Direction::Right,
            pos: (y, -1),
        }));
    }
    for y in 0..width {
        brightness = brightness.max(Field::from(&types).count(Beam {
            dir: Direction::Left,
            pos: (y, width),
        }));
    }
    for x in 0..width {
        brightness = brightness.max(Field::from(&types).count(Beam {
            dir: Direction::Bottom,
            pos: (-1, x),
        }));
    }
    for x in 0..width {
        brightness = brightness.max(Field::from(&types).count(Beam {
            dir: Direction::Top,
            pos: (height, x),
        }));
    }

    brightness
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

        assert_eq!(res, 51);
    }
}
