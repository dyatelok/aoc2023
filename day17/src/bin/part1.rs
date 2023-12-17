use itertools::*;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
enum Dir {
    L,
    R,
    T,
    B,
}

impl Dir {
    fn is_opposite(&self, other: Self) -> bool {
        matches!(
            (*self, other),
            (Dir::L, Dir::R) | (Dir::R, Dir::L) | (Dir::T, Dir::B) | (Dir::B, Dir::T)
        )
    }
}

fn not_opposite(d1: Option<Dir>, d2: Option<Dir>) -> bool {
    match (d1, d2) {
        (_, None) => true,
        (None, _) => true,
        (Some(d1), Some(d2)) => !d1.is_opposite(d2),
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
struct State {
    pos: (i32, i32),
    previous_dir: Option<Dir>,
    dir_duration: usize,
}

impl State {
    fn from(pos: (i32, i32), previous_dir: Option<Dir>, dir_duration: usize) -> Self {
        Self {
            pos,
            previous_dir,
            dir_duration,
        }
    }
}

struct Field {
    field: Vec<Vec<usize>>,
    states: HashMap<State, usize>,
    width: i32,
    height: i32,
}

use std::ops::Add;
fn add<T: Add<Output = T>>(a: (T, T), b: (T, T)) -> (T, T) {
    (a.0 + b.0, a.1 + b.1)
}

impl Field {
    fn from(field: Vec<Vec<usize>>) -> Self {
        let width = field[0].len();
        let height = field.len();
        Field {
            field,
            states: HashMap::new(),
            width: width as i32,
            height: height as i32,
        }
    }
    fn solve(mut self) -> usize {
        let start = State::from((0, 0), None, 0);
        let _ = self.states.insert(start, 0);
        let mut active = HashSet::new();
        active.insert((start, 0));

        while !active.is_empty() {
            // println!("{}", active.len());
            active = active
                .into_iter()
                .flat_map(|(state, loss)| {
                    let new_states = [
                        ((1, 0), Dir::B),
                        ((-1, 0), Dir::T),
                        ((0, 1), Dir::R),
                        ((0, -1), Dir::L),
                    ]
                    .into_iter()
                    .flat_map(|(mov, dir)| {
                        let new_pos = add(mov, state.pos);

                        if (0..self.height).contains(&new_pos.0)
                            && (0..self.width).contains(&new_pos.1)
                        {
                            let new_dir = Some(dir);
                            let new_dir_duration = if state.previous_dir == new_dir {
                                state.dir_duration + 1
                            } else {
                                1
                            };
                            let new_loss =
                                loss + self.field[new_pos.0 as usize][new_pos.1 as usize];
                            if new_dir_duration < 4 && not_opposite(state.previous_dir, new_dir) {
                                Some((State::from(new_pos, new_dir, new_dir_duration), new_loss))
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    })
                    .collect_vec();

                    new_states
                        .into_iter()
                        .flat_map(|(state, loss)| {
                            if let Some(prev_loss) = self.states.get(&state) {
                                if loss < *prev_loss {
                                    let _ = self.states.insert(state, loss);
                                    Some((state, loss))
                                } else {
                                    None
                                }
                            } else {
                                self.states.insert(state, loss);
                                Some((state, loss))
                            }
                        })
                        .collect_vec()
                })
                .collect();
        }

        self.states
            .into_iter()
            .filter_map(|(state, loss)| {
                if state.pos == (self.height - 1, self.width - 1) {
                    Some(loss)
                } else {
                    None
                }
            })
            // .map(|(pos, loss)| {
            //     println!("{pos:?}: {loss}");
            //     loss
            // })
            .min()
            .unwrap()
    }
}

fn solution(input: &str) -> usize {
    let field = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_string().parse::<usize>().unwrap())
                .collect_vec()
        })
        .collect_vec();

    Field::from(field).solve()
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
        let input = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
        let res = solution(input);

        assert_eq!(res, 102);
    }
}
