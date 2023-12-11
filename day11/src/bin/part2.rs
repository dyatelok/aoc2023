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

use itertools::*;

fn solution(input: &str, factor: usize) -> usize {
    let space = input
        .lines()
        .map(|line| line.trim().chars().map(Tile::from).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut galaxies = vec![];
    for (i, row) in space.iter().enumerate() {
        for (j, tile) in row.iter().enumerate() {
            if *tile == Tile::Galaxy {
                galaxies.push((i as i32, j as i32));
            }
        }
    }
    let total = galaxies.len();

    let mut distances = 0;

    galaxies
        .iter()
        .cartesian_product(galaxies.iter())
        .for_each(|((x1, y1), (x2, y2))| {
            distances += (x1 - x2).abs() + (y1 - y2).abs();
        });

    let mut distances = distances as usize / 2;

    let mut passed = 0;

    for row in space.iter() {
        row.iter().for_each(|tile| {
            if *tile == Tile::Galaxy {
                passed += 1;
            }
        });
        if !row.contains(&Tile::Galaxy) {
            distances += passed * (total - passed) * (factor - 1);
        }
    }

    let space = transpose(space);

    let mut passed = 0;
    for row in space.iter() {
        row.iter().for_each(|tile| {
            if *tile == Tile::Galaxy {
                passed += 1;
            }
        });
        if !row.contains(&Tile::Galaxy) {
            distances += passed * (total - passed) * (factor - 1);
        }
    }

    distances
}

fn main() {
    let input = include_str!("../../input1.txt");
    let res = solution(input, 1_000_000);
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
        let res = solution(input, 10);
        assert_eq!(res, 1030);
    }
    #[test]
    fn test2() {
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
        let res = solution(input, 100);
        assert_eq!(res, 8410);
    }
    #[test]
    fn test3() {
        let input = "...#
....
....
#...";
        let res = solution(input, 10);
        assert_eq!(res, 42);
    }
    #[test]
    fn test4() {
        let input = "#..#
....
....
#..#";
        let res = solution(input, 10);
        assert_eq!(res, 168);
    }
}
