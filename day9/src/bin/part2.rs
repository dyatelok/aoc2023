use itertools::*;

fn solution(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            let mut arr = line
                .split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            let mut firsts = vec![];

            while arr.iter().any(|n| *n != 0) {
                firsts.push(*arr.first().unwrap());
                arr = arr.iter().tuple_windows().map(|(a, b)| b - a).collect();
            }

            firsts.reverse();
            firsts.into_iter().fold(0, |acc, elem| elem - acc)
        })
        .sum()
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
        let input = "
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        let res = solution(input);
        assert_eq!(res, 2);
    }
}
