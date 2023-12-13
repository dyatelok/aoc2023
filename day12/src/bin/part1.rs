use std::fmt;

#[derive(PartialEq, Eq, Clone, Copy)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

impl fmt::Debug for Spring {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Damaged => '#',
                Self::Operational => '.',
                Self::Unknown => '?',
            }
        )
    }
}

impl From<char> for Spring {
    fn from(value: char) -> Self {
        match value {
            '#' => Spring::Damaged,
            '.' => Spring::Operational,
            '?' => Self::Unknown,
            _ => panic!("{value}"),
        }
    }
}

fn solution(input: &str) -> usize {
    input.lines().map(handle_line).sum()
}

fn handle_line(line: &str) -> usize {
    let (springs, nums) = line.split_at(line.find(' ').unwrap());
    let mut springs = springs.trim().chars().map(Spring::from).collect::<Vec<_>>();
    let nums = nums
        .trim()
        .split(',')
        .map(|num| {
            num.parse::<usize>().unwrap_or_else(|_| {
                panic!("'{num}'");
            })
        })
        .collect::<Vec<_>>();

    // println!("{:?} {:?}\n", springs, nums);

    springs.push(Spring::Operational);

    let length = springs.len();

    let mut possible = vec![springs];

    for iteration in 0..length {
        possible = iterate(iteration, possible, &nums);

        // for p in possible.iter() {
        //     println!("{p:?}");
        // }
        // println!();
    }

    possible.len()
}

fn iterate(iteration: usize, possible: Vec<Vec<Spring>>, nums: &Vec<usize>) -> Vec<Vec<Spring>> {
    let mut new = vec![];

    for sequence in possible {
        if sequence[iteration] == Spring::Unknown {
            let mut seq_o = sequence.clone();
            seq_o[iteration] = Spring::Operational;
            if passes(&seq_o, nums) {
                new.push(seq_o);
            }

            let mut seq_d = sequence;
            seq_d[iteration] = Spring::Damaged;
            if passes(&seq_d, nums) {
                new.push(seq_d);
            }
        } else {
            new.push(sequence);
        }
    }

    new
}

fn passes(sequence: &Vec<Spring>, nums: &Vec<usize>) -> bool {
    let mut ptr = 0;
    let mut accu = 0;

    let mut prev = None;

    for s in sequence {
        match s {
            Spring::Unknown => {
                return true;
            }
            Spring::Damaged => {
                accu += 1;
            }
            Spring::Operational => {
                if let Some(Spring::Damaged) = prev {
                    if ptr >= nums.len() {
                        return false;
                    }
                    if nums[ptr] == accu {
                        ptr += 1;
                        accu = 0;
                    } else {
                        return false;
                    }
                }
            }
        }
        prev = Some(*s);
    }

    ptr == nums.len()
}

fn main() {
    let input = include_str!("../../input1.txt");
    let res = solution(input);
    println!("{res}");
}

#[cfg(test)]
mod tests {
    use super::solution;
    use crate::handle_line;

    #[test]
    fn test1() {
        let line = "???.### 1,1,3";
        let res = handle_line(line);
        assert_eq!(res, 1);
    }
    #[test]
    fn test2() {
        let line = ".??..??...?##. 1,1,3";
        let res = handle_line(line);
        assert_eq!(res, 4);
    }
    #[test]
    fn test3() {
        let line = "?#?#?#?#?#?#?#? 1,3,1,6";
        let res = handle_line(line);
        assert_eq!(res, 1);
    }
    #[test]
    fn test4() {
        let line = "????.#...#... 4,1,1";
        let res = handle_line(line);
        assert_eq!(res, 1);
    }
    #[test]
    fn test5() {
        let line = "????.######..#####. 1,6,5";
        let res = handle_line(line);
        assert_eq!(res, 4);
    }
    #[test]
    fn test6() {
        let line = "?###???????? 3,2,1";
        let res = handle_line(line);
        assert_eq!(res, 10);
    }
    #[test]
    fn test7() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        let res = solution(input);
        assert_eq!(res, 21);
    }
}
