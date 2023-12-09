enum Way {
    Left,
    Right,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Node([u8; 3]);

impl Node {
    fn from(inp: &str) -> Self {
        let arr: [u8; 3] = inp.as_bytes().try_into().unwrap();
        Self(arr)
    }
}

use std::collections::HashMap;
use std::iter::*;

fn solution(input: &str) -> u32 {
    let mut lines = input.lines();
    let mut directions = lines
        .next()
        .unwrap()
        .chars()
        .map(|ch| match ch {
            'L' => Way::Left,
            'R' => Way::Right,
            _ => panic!("Unknown item in the directions squence!"),
        })
        .cycle();

    let _ = lines.next();

    let paths: HashMap<_, _> = lines
        .map(|line| {
            let cleaned = line.replace(&['=', '(', ',', ')'][..], "");
            let mut iter = cleaned.split_whitespace();

            let key = Node::from(iter.next().unwrap());
            let left = Node::from(iter.next().unwrap());
            let right = Node::from(iter.next().unwrap());
            (key, (left, right))
        })
        .collect();

    let mut current = Node::from("AAA");
    let target = Node::from("ZZZ");
    let mut counter = 0;

    while current != target {
        counter += 1;
        let dir = directions.next().unwrap();
        let (left, right) = paths.get(&current).unwrap();
        current = match dir {
            Way::Left => *left,
            Way::Right => *right,
        }
    }

    counter
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
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        let res = solution(input);
        assert_eq!(res, 6);
    }
}
