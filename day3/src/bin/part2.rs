use std::collections::HashMap;

fn solution(input: &str) -> u32 {
    let w = input.lines().next().unwrap().len() as i32;
    let h = input.lines().count() as i32;

    let mut numbers: Vec<(i32, i32, i32, u32)> = vec![];

    input.lines().enumerate().for_each(|(i, line)| {
        let mut num: Option<(usize, usize, u32)> = None;
        line.chars()
            .enumerate()
            .for_each(|(pos_j, symbol)| match symbol.to_digit(10) {
                None => {
                    if let Some((j, len, nm)) = num {
                        numbers.push((i as i32, j as i32, len as i32, nm));
                        num = None;
                    }
                }
                Some(dig) => {
                    num = match num {
                        None => Some((pos_j, 1, dig)),
                        Some((j, len, prev)) => Some((j, len + 1, prev * 10 + dig)),
                    };
                }
            });
        if let Some((j, len, n)) = num {
            numbers.push((i as i32, j as i32, len as i32, n));
        }
    });

    let table = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut gears: HashMap<(usize, usize), Vec<u32>> = HashMap::new();

    numbers.into_iter().for_each(|(i, j, l, n)| {
        for row in i - 1..=i + 1 {
            for col in j - 1..=j + l {
                if (0..w).contains(&col) && (0..h).contains(&row) {
                    let cords = (row as usize, col as usize);
                    if table[cords.0][cords.1] == '*' {
                        gears
                            .entry(cords)
                            .and_modify(|vec| {
                                vec.push(n);
                            })
                            .or_insert(vec![n]);
                    }
                }
            }
        }
    });

    // println!("{:#?}", gears);

    gears
        .into_iter()
        .map(|(_, vec)| if vec.len() == 2 { vec[0] * vec[1] } else { 0 })
        .sum::<u32>()
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
    // #[ignore]
    fn test1() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let res = solution(input);
        assert_eq!(res, 467835);
    }

    #[test]
    // #[ignore]
    fn test2() {
        let input = "467..114..
...*......
.664.598..";
        let res = solution(input);
        assert_eq!(res, 467 * 664);
    }
    #[test]
    // #[ignore]
    fn test3() {
        let input = "*467
.123";
        let res = solution(input);
        assert_eq!(res, 467 * 123);
    }
}
