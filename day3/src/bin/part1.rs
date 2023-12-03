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

    // println!("numbers: {:#?}", numbers);

    numbers
        .into_iter()
        .filter_map(|(i, j, l, n)| {
            for row in i - 1..=i + 1 {
                for col in j - 1..=j + l {
                    // println!("row: {}, col: {}", row, col);
                    if (0..w).contains(&col) && (0..h).contains(&row) {
                        let (rw, cl) = (row as usize, col as usize);
                        if table[rw][cl] != '.' && !table[rw][cl].is_ascii_digit() {
                            return Some(n);
                        }
                    }
                }
            }
            None
        })
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
        assert_eq!(res, 4361);
    }

    #[test]
    // #[ignore]
    fn test2() {
        let input = "467..114..
...*......
.664.598..";
        let res = solution(input);
        assert_eq!(res, 467 + 664);
    }
    #[test]
    fn test3() {
        let input = "*467
*123";
        let res = solution(input);
        assert_eq!(res, 467 + 123);
    }
}
