enum Way {
    Left,
    Right,
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Node([u8; 3]);

impl Node {
    fn from(inp: &str) -> Self {
        let arr: [u8; 3] = inp.as_bytes().try_into().unwrap();
        Self(arr)
    }
}

use std::collections::HashMap;
use std::iter::*;

fn solution(input: &str) -> u128 {
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

    let nodes = lines
        .map(|line| {
            let cleaned = line.replace(&['=', '(', ',', ')'][..], "");
            let mut iter = cleaned.split_whitespace();

            let key = Node::from(iter.next().unwrap());
            let left = Node::from(iter.next().unwrap());
            let right = Node::from(iter.next().unwrap());
            [key, left, right]
        })
        .collect::<Vec<_>>();

    let paths: HashMap<_, _> = nodes
        .clone()
        .into_iter()
        .map(|[key, left, right]| (key, (left, right)))
        .collect();

    let mut nodes = nodes
        .into_iter()
        .map(|[first, _, _]| first)
        .collect::<Vec<_>>();

    nodes.sort_unstable();
    nodes.dedup();
    nodes.retain(|node| node.0[2] == b'A');
    // println!("nodes: {:?}", nodes);

    // This thing is wrong and it shpuldn't work, but the input is broken
    nodes
        .into_iter()
        .map(|mut node| {
            let mut directions_cloned = directions.clone();
            let mut counter = 0;

            while node.0[2] != b'Z' {
                counter += 1;
                let dir = directions_cloned.next().unwrap();
                let (left, right) = paths.get(&node).unwrap();
                node = match dir {
                    Way::Left => *left,
                    Way::Right => *right,
                }
            }

            counter
        })
        .fold(1, lcm)
}

fn lcm(a: u128, b: u128) -> u128 {
    a * b / gcd(a, b)
}

fn gcd(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        let tmp = a;
        a = b;
        b = tmp % b;
    }
    a
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
    fn test2() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        let res = solution(input);
        assert_eq!(res, 6);
    }
}
