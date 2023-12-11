#[derive(PartialEq, Eq, Clone, Copy)]
enum Tile {
    Galaxy,
    Empty,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '#' => Tile::Galaxy,
            '.' => Tile::Empty,
            _ => panic!(),
        }
    }
}

#[allow(unused)]
fn debug(space: &Vec<Vec<Tile>>) {
    for row in space {
        for tile in row {
            print!(
                "{}",
                match tile {
                    Tile::Galaxy => '#',
                    Tile::Empty => '.',
                }
            );
        }
        println!();
    }
    println!();
}

fn transpose(orig: Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
    let mut transposed = (0..orig[0].len()).map(|_| vec![]).collect::<Vec<_>>();

    for orow in orig {
        for (item, trow) in orow.into_iter().zip(&mut transposed) {
            trow.push(item);
        }
    }

    transposed
}

fn expand_rows(space: Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
    space
        .into_iter()
        .flat_map(|row| {
            if row.contains(&Tile::Galaxy) {
                vec![row]
            } else {
                vec![row.clone(), row]
            }
        })
        .collect()
}

use itertools::*;

fn solution(input: &str) -> usize {
    let space = input
        .lines()
        .map(|line| line.trim().chars().map(Tile::from).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let space = expand_rows(space);
    let space = transpose(space);
    let space = expand_rows(space);
    let space = transpose(space);

    let mut galaxies = vec![];
    for (i, row) in space.into_iter().enumerate() {
        for (j, tile) in row.into_iter().enumerate() {
            if tile == Tile::Galaxy {
                galaxies.push((i as i32, j as i32));
            }
        }
    }

    let mut distances = 0;

    galaxies
        .iter()
        .cartesian_product(galaxies.iter())
        .for_each(|((x1, y1), (x2, y2))| {
            distances += (x1 - x2).abs() + (y1 - y2).abs();
        });

    distances as usize / 2
}

fn main() {
    let input = include_str!("../../input1.txt");
    let res = solution(input);
    println!("{res}");
}

#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    fn test1() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        let res = solution(input);
        assert_eq!(res, 374);
    }
}
