#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Symbol {
    visit: Option<usize>,
    top: bool,
    bottom: bool,
    left: bool,
    right: bool,
}

#[allow(unused, clippy::ptr_arg)]
fn debug(map: &Vec<Vec<Symbol>>) {
    for vv in map.iter() {
        for v in vv {
            print!(
                "{}",
                match v.visit {
                    None => String::from('.'),
                    Some(val) => val.clone().to_string(),
                }
            );
        }
        println!();
    }
    // for vv in map.iter() {
    //     for v in vv {
    //         print!("{:?}", v);
    //     }
    //     println!();
    // }
}

impl From<char> for Symbol {
    fn from(value: char) -> Self {
        match value {
            '.' => Self {
                visit: None,
                top: false,
                bottom: false,
                left: false,
                right: false,
            },
            '|' => Self {
                visit: None,
                top: true,
                bottom: true,
                left: false,
                right: false,
            },
            '-' => Self {
                visit: None,
                top: false,
                bottom: false,
                left: true,
                right: true,
            },
            'L' => Self {
                visit: None,
                top: true,
                bottom: false,
                left: false,
                right: true,
            },
            'J' => Self {
                visit: None,
                top: true,
                bottom: false,
                left: true,
                right: false,
            },
            '7' => Self {
                visit: None,
                top: false,
                bottom: true,
                left: true,
                right: false,
            },
            'F' => Self {
                visit: None,
                top: false,
                bottom: true,
                left: false,
                right: true,
            },
            'S' => Self {
                visit: None,
                top: true,
                bottom: true,
                left: true,
                right: true,
            },
            _ => panic!(),
        }
    }
}

fn solution(input: &str) -> usize {
    let mut map = input
        .lines()
        .map(|line| line.trim().chars().map(Symbol::from).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let row = map.first().unwrap().len();
    let col = map.len();

    let (num, _) = input
        .lines()
        .flat_map(|line| line.chars())
        .enumerate()
        .find(|(_, sym)| *sym == 'S')
        .unwrap();
    let start = (num / row, num % row);

    map[start.0][start.1].visit = Some(0);
    let mut starting = vec![start];

    let mut ctr = 0;

    while !starting.is_empty() {
        ctr += 1;
        // println!("{:?}", starting);
        // debug(&map);
        // println!();

        let mut new_pos = vec![];
        for elem in starting {
            if (0..col).contains(&((elem.0 as i32 - 1) as usize))
                && (0..row).contains(&elem.1)
                && map[elem.0][elem.1].top
                && map[elem.0 - 1][elem.1].bottom
                && map[elem.0 - 1][elem.1].visit.is_none()
            {
                new_pos.push((elem.0 - 1, elem.1));
                map[elem.0 - 1][elem.1].visit = Some(ctr);
            }
            if (0..col).contains(&(elem.0 + 1))
                && (0..row).contains(&elem.1)
                && map[elem.0][elem.1].bottom
                && map[elem.0 + 1][elem.1].top
                && map[elem.0 + 1][elem.1].visit.is_none()
            {
                new_pos.push((elem.0 + 1, elem.1));
                map[elem.0 + 1][elem.1].visit = Some(ctr);
            }
            if (0..col).contains(&elem.0)
                && (0..row).contains(&((elem.1 as i32 - 1) as usize))
                && map[elem.0][elem.1].left
                && map[elem.0][elem.1 - 1].right
                && map[elem.0][elem.1 - 1].visit.is_none()
            {
                new_pos.push((elem.0, elem.1 - 1));
                map[elem.0][elem.1 - 1].visit = Some(ctr);
            }
            if (0..col).contains(&elem.0)
                && (0..row).contains(&(elem.1 + 1))
                && map[elem.0][elem.1].right
                && map[elem.0][elem.1 + 1].left
                && map[elem.0][elem.1 + 1].visit.is_none()
            {
                new_pos.push((elem.0, elem.1 + 1));
                map[elem.0][elem.1 + 1].visit = Some(ctr);
            }
        }
        starting = new_pos;
    }

    // debug(&map);

    ctr - 1
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
        let input = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";
        let res = solution(input);
        assert_eq!(res, 8);
    }
    #[test]
    fn test2() {
        let input = ".....
.S-7.
.|.|.
.L-J.
.....";
        let res = solution(input);
        assert_eq!(res, 4);
    }
    #[test]
    fn test3() {
        let input = "F-7..
|.|..
L-S-7
..|.|
..L-J";
        let res = solution(input);
        assert_eq!(res, 4);
    }
}
