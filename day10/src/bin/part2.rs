#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Symbol {
    visit: bool,
    top: bool,
    bottom: bool,
    left: bool,
    right: bool,
}

#[allow(unused, clippy::ptr_arg)]
fn debug_field(field: &Vec<Vec<bool>>) {
    for vv in field.into_iter() {
        for v in vv {
            print!(
                "{}",
                match v {
                    false => '.',
                    true => '#',
                }
            );
        }
        println!();
    }
}

#[allow(unused, clippy::ptr_arg)]
fn debug_char(spread: &Vec<Vec<char>>) {
    for row in spread.iter() {
        for sym in row {
            print!("{sym}");
        }
        println!();
    }
    println!();
}

impl From<char> for Symbol {
    fn from(value: char) -> Self {
        match value {
            '.' => Self {
                visit: false,
                top: false,
                bottom: false,
                left: false,
                right: false,
            },
            '|' => Self {
                visit: false,
                top: true,
                bottom: true,
                left: false,
                right: false,
            },
            '-' => Self {
                visit: false,
                top: false,
                bottom: false,
                left: true,
                right: true,
            },
            'L' => Self {
                visit: false,
                top: true,
                bottom: false,
                left: false,
                right: true,
            },
            'J' => Self {
                visit: false,
                top: true,
                bottom: false,
                left: true,
                right: false,
            },
            '7' => Self {
                visit: false,
                top: false,
                bottom: true,
                left: true,
                right: false,
            },
            'F' => Self {
                visit: false,
                top: false,
                bottom: true,
                left: false,
                right: true,
            },
            'S' => Self {
                visit: false,
                top: true,
                bottom: true,
                left: true,
                right: true,
            },
            _ => panic!(),
        }
    }
}

fn spread(input: &str) -> (usize, usize, Vec<Vec<char>>) {
    let map = input
        .lines()
        .map(|line| line.trim().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let cols = map.first().unwrap().len();
    let rows = map.len();

    let mut spread = vec![vec!['.'; cols * 2 + 1]; rows * 2 + 1];

    for (rown, row) in map.into_iter().enumerate() {
        for (coln, elem) in row.into_iter().enumerate() {
            spread[rown * 2 + 1][coln * 2 + 1] = elem;
        }
    }
    // debug_char(&spread);

    for i in 0..rows - 1 {
        let rown = i * 2 + 2;
        for j in 0..cols {
            let coln = j * 2 + 1;
            if Symbol::from(spread[rown - 1][coln]).bottom
                && Symbol::from(spread[rown + 1][coln]).top
            {
                spread[rown][coln] = '|';
            }
        }
    }

    for j in 0..cols - 1 {
        let coln = j * 2 + 2;
        for i in 0..rows {
            let rown = i * 2 + 1;
            if Symbol::from(spread[rown][coln - 1]).right
                && Symbol::from(spread[rown][coln + 1]).left
            {
                spread[rown][coln] = '-';
            }
        }
    }

    // debug_char(&spread);

    (rows, cols, spread)
}

fn solution(input: &str) -> usize {
    let (irows, icols, spreaded) = spread(input);

    let (num, _) = spreaded
        .iter()
        .flatten()
        .enumerate()
        .find(|(_, sym)| **sym == 'S')
        .unwrap();
    let row = spreaded.first().unwrap().len();
    let col = spreaded.len();
    let start = (num / row, num % row);

    let mut map = spreaded
        .into_iter()
        .map(|row| row.into_iter().map(Symbol::from).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    map[start.0][start.1].visit = true;
    let mut starting = vec![start];

    while !starting.is_empty() {
        let mut new_pos = vec![];
        for elem in starting {
            if (0..col).contains(&((elem.0 as i32 - 1) as usize))
                && (0..row).contains(&elem.1)
                && map[elem.0][elem.1].top
                && map[elem.0 - 1][elem.1].bottom
                && !map[elem.0 - 1][elem.1].visit
            {
                new_pos.push((elem.0 - 1, elem.1));
                map[elem.0 - 1][elem.1].visit = true;
            }
            if (0..col).contains(&(elem.0 + 1))
                && (0..row).contains(&elem.1)
                && map[elem.0][elem.1].bottom
                && map[elem.0 + 1][elem.1].top
                && !map[elem.0 + 1][elem.1].visit
            {
                new_pos.push((elem.0 + 1, elem.1));
                map[elem.0 + 1][elem.1].visit = true;
            }
            if (0..col).contains(&elem.0)
                && (0..row).contains(&((elem.1 as i32 - 1) as usize))
                && map[elem.0][elem.1].left
                && map[elem.0][elem.1 - 1].right
                && !map[elem.0][elem.1 - 1].visit
            {
                new_pos.push((elem.0, elem.1 - 1));
                map[elem.0][elem.1 - 1].visit = true;
            }
            if (0..col).contains(&elem.0)
                && (0..row).contains(&(elem.1 + 1))
                && map[elem.0][elem.1].right
                && map[elem.0][elem.1 + 1].left
                && !map[elem.0][elem.1 + 1].visit
            {
                new_pos.push((elem.0, elem.1 + 1));
                map[elem.0][elem.1 + 1].visit = true;
            }
        }
        starting = new_pos;
    }

    let mut field = map
        .into_iter()
        .map(|line| line.into_iter().map(|v| v.visit).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // debug_field(&field);

    let row = field.first().unwrap().len();
    let col = field.len();

    let mut points = vec![(0, 0)];

    while !points.is_empty() {
        let mut points2 = vec![];

        for point in points {
            if (0..col).contains(&((point.0 as i32 - 1) as usize))
                && (0..row).contains(&(point.1 as usize))
                && !field[point.0 - 1][point.1]
            {
                field[point.0 - 1][point.1] = true;
                points2.push((point.0 - 1, point.1));
            }
            if (0..col).contains(&(point.0 as usize))
                && (0..row).contains(&((point.1 as i32 - 1) as usize))
                && !field[point.0][point.1 - 1]
            {
                field[point.0][point.1 - 1] = true;
                points2.push((point.0, point.1 - 1));
            }
            if (0..col).contains(&((point.0 + 1) as usize))
                && (0..row).contains(&(point.1 as usize))
                && !field[point.0 + 1][point.1]
            {
                field[point.0 + 1][point.1] = true;
                points2.push((point.0 + 1, point.1));
            }
            if (0..col).contains(&(point.0 as usize))
                && (0..row).contains(&((point.1 + 1) as usize))
                && !field[point.0][point.1 + 1]
            {
                field[point.0][point.1 + 1] = true;
                points2.push((point.0, point.1 + 1));
            }
        }
        points = points2;
    }

    let mut ctr = 0;

    for i in 0..irows {
        let row = i * 2 + 1;
        for j in 0..icols {
            let col = j * 2 + 1;
            if !field[row][col] {
                ctr += 1;
            }
        }
    }

    ctr
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
        let input = ".....
.S-7.
.|.|.
.L-J.
.....";
        let res = solution(input);
        assert_eq!(res, 1);
    }
    #[test]
    fn test2() {
        let input = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";
        let res = solution(input);
        assert_eq!(res, 1);
    }
    #[test]
    fn test3() {
        let input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
        let res = solution(input);
        assert_eq!(res, 4);
    }
    #[test]
    fn test4() {
        let input = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
        let res = solution(input);
        assert_eq!(res, 8);
    }
    #[test]
    fn tes5() {
        let input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
        let res = solution(input);
        assert_eq!(res, 10);
    }
}
