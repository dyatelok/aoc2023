fn split_on(line: &str, mid: usize) -> (&str, &str) {
    (&line[..mid], &line[mid + 1..])
}

fn parse(nums: &str) -> Vec<u32> {
    nums.split_whitespace()
        .map(|num| num.parse::<u32>().unwrap())
        .collect()
}

fn solution(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (_, nums) = split_on(line, line.find(':').unwrap() + 1);
            let (winning, mine) = split_on(nums, nums.find('|').unwrap());
            let (winning, mine) = (parse(winning), parse(mine));
            let ws = winning.iter();
            let ms = mine.iter();

            ms.flat_map(|y| ws.clone().map(move |x| (x, y)))
                .fold(0, |res, (a, b)| {
                    if a == b {
                        match res {
                            0 => 1,
                            n => 2 * n,
                        }
                    } else {
                        res
                    }
                })
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
    fn test1() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let res = solution(input);
        assert_eq!(res, 13);
    }
}
