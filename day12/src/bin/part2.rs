#![feature(iter_intersperse)]
use std::fmt;

#[derive(PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
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
    let pre_springs = springs.trim().chars().map(Spring::from).collect::<Vec<_>>();
    let pre_nums = nums
        .trim()
        .split(',')
        .map(|num| {
            num.parse::<usize>().unwrap_or_else(|_| {
                panic!("'{num}'");
            })
        })
        .collect::<Vec<_>>();

    let mut springs = std::iter::repeat(pre_springs)
        .take(5)
        .intersperse(vec![Spring::Unknown])
        .flatten()
        .collect::<Vec<_>>();

    let nums = std::iter::repeat(pre_nums)
        .take(5)
        .flatten()
        .collect::<Vec<_>>();

    springs.push(Spring::Operational);

    let mut groups = vec![Params::from(0, 0, 1, None)];

    for s in springs {
        let mut new_groups = vec![];
        for g in groups {
            match s {
                Spring::Unknown => {
                    damaged(&mut new_groups, &g);
                    operational(&mut new_groups, &g, &nums);
                }
                Spring::Damaged => damaged(&mut new_groups, &g),
                Spring::Operational => operational(&mut new_groups, &g, &nums),
            }
        }

        new_groups.sort();

        groups = new_groups.into_iter().fold(Vec::new(), |mut vec, elem| {
            if let Some(last) = vec.last_mut() {
                if last.seq == elem.seq && last.ptr == elem.ptr && last.prev == elem.prev {
                    last.amt += elem.amt;
                    return vec;
                }
            }
            vec.push(elem);
            vec
        });
    }

    groups
        .into_iter()
        .filter_map(|g| {
            if g.ptr == nums.len() {
                Some(g.amt)
            } else {
                None
            }
        })
        .sum()
}

fn damaged(new_groups: &mut Vec<Params>, g: &Params) {
    new_groups.push(Params::from(g.seq + 1, g.ptr, g.amt, Some(Spring::Damaged)));
}

fn operational(new_groups: &mut Vec<Params>, g: &Params, nums: &Vec<usize>) {
    if g.prev == Some(Spring::Damaged) {
        if g.ptr < nums.len() && nums[g.ptr] == g.seq {
            new_groups.push(Params::from(0, g.ptr + 1, g.amt, Some(Spring::Operational)));
        }
    } else {
        new_groups.push(Params::from(0, g.ptr, g.amt, Some(Spring::Operational)));
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Params {
    seq: usize,
    ptr: usize,
    amt: usize,
    prev: Option<Spring>,
}

impl Params {
    fn from(seq: usize, ptr: usize, amt: usize, prev: Option<Spring>) -> Self {
        Self {
            seq,
            ptr,
            amt,
            prev,
        }
    }
}

fn main() {
    let input = include_str!("../../input1.txt");
    let res = solution(input);
    println!("{res}");
}

#[cfg(test)]
mod tests {
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
        assert_eq!(res, 16384);
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
        assert_eq!(res, 16);
    }
    #[test]
    fn test5() {
        let line = "????.######..#####. 1,6,5";
        let res = handle_line(line);
        assert_eq!(res, 2500);
    }
    #[test]
    fn test6() {
        let line = "?###???????? 3,2,1";
        let res = handle_line(line);
        assert_eq!(res, 506250);
    }
}
