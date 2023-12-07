#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Type {
    HighKard,
    OnePair,
    TwoPairs,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Kard(char);

impl From<char> for Kard {
    fn from(value: char) -> Self {
        Self(value)
    }
}

fn rank(value: char) -> u8 {
    match value {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 0,
        'T' => 10,
        ch => ch as u8 - b'0',
    }
}

use std::cmp::Ordering;
impl PartialOrd for Kard {
    fn partial_cmp(&self, other: &Kard) -> Option<Ordering> {
        Some(rank(self.0).cmp(&rank(other.0)))
    }
}

impl Ord for Kard {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord)]
struct Hand {
    ty: Type,
    comb: [Kard; 5],
}

impl Hand {
    fn from(inp: &str) -> Self {
        let mut comb = inp.chars().map(Kard::from).collect::<Vec<_>>();
        let jokers = comb.iter().filter(|kard| kard.0 == 'J').count();
        comb.sort();
        let comb: [Kard; 5] = comb.try_into().unwrap();

        let ty = match jokers {
            0 => match comb {
                [a, b, c, d, e] if a == b && b == c && c == d && d == e => Type::FiveOfKind,
                [a, b, c, d, _] if a == b && b == c && c == d => Type::FourOfKind,
                [_, b, c, d, e] if b == c && c == d && d == e => Type::FourOfKind,
                [a, b, c, d, e] if (a == b && b == c) && d == e => Type::FullHouse,
                [a, b, c, d, e] if a == b && (c == d && d == e) => Type::FullHouse,
                [a, b, c, _, _] if a == b && b == c => Type::ThreeOfKind,
                [_, b, c, d, _] if b == c && c == d => Type::ThreeOfKind,
                [_, _, c, d, e] if c == d && d == e => Type::ThreeOfKind,
                [a, b, c, d, _] if (a == b) && (c == d) => Type::TwoPairs,
                [a, b, _, d, e] if (a == b) && (d == e) => Type::TwoPairs,
                [_, b, c, d, e] if (b == c) && (d == e) => Type::TwoPairs,
                [a, b, _, _, _] if a == b => Type::OnePair,
                [_, b, c, _, _] if b == c => Type::OnePair,
                [_, _, c, d, _] if c == d => Type::OnePair,
                [_, _, _, d, e] if d == e => Type::OnePair,
                _ => Type::HighKard,
            },
            1 => match comb {
                [_, a, b, c, d] if a == b && b == c && c == d => Type::FiveOfKind,
                [_, a, b, c, _] if a == b && b == c => Type::FourOfKind,
                [_, _, b, c, d] if b == c && c == d => Type::FourOfKind,
                [_, a, b, c, d] if a == b && c == d => Type::FullHouse,
                [_, a, b, _, _] if a == b => Type::ThreeOfKind,
                [_, _, b, c, _] if b == c => Type::ThreeOfKind,
                [_, _, _, c, d] if c == d => Type::ThreeOfKind,
                _ => Type::OnePair,
            },
            2 => match comb {
                [_, _, a, b, c] if a == b && b == c => Type::FiveOfKind,
                [_, _, a, b, _] if a == b => Type::FourOfKind,
                [_, _, _, b, c] if b == c => Type::FourOfKind,
                _ => Type::ThreeOfKind,
            },
            3 => match comb {
                [_, _, _, a, b] if a == b => Type::FiveOfKind,
                _ => Type::FourOfKind,
            },
            4 | 5 => Type::FiveOfKind,
            _ => panic!(),
        };

        let comb = inp
            .chars()
            .map(Kard::from)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        Self { ty, comb }
    }
}

fn get_data(input: &str) -> (Hand, u128) {
    let (hand, val) = input.split_at(5);

    (Hand::from(hand), val.trim().parse().unwrap())
}

fn solution(input: &str) -> u128 {
    let mut hands = input.lines().map(get_data).collect::<Vec<_>>();

    hands.sort_by_key(|(hand, _)| *hand);

    // println!("{:#?}", hands);

    hands
        .into_iter()
        .enumerate()
        .map(|(i, (_, val))| (i + 1) as u128 * val)
        .sum()
}

fn main() {
    let input = include_str!("../../input1.txt");
    let res = solution(input);
    println!("{res}");
}

#[cfg(test)]
mod tests {
    use crate::{Hand, Type};

    use super::solution;

    #[test]
    fn test1() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        let res = solution(input);
        assert_eq!(res, 5905);
    }

    #[test]
    fn teste() {
        let input = "6JTJJ";
        let res = Hand::from(input);
        assert_eq!(res.ty, Type::FourOfKind);
    }
}
