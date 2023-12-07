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
        'J' => 11,
        'T' => 10,
        _ => value as u8 - b'0',
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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Hand {
    ty: Type,
    comb: [Kard; 5],
}

impl Hand {
    fn from(inp: &str) -> Self {
        let mut comb = inp.chars().map(Kard::from).collect::<Vec<_>>();
        comb.sort_unstable();
        let comb: [Kard; 5] = comb.try_into().unwrap();

        let ty = match comb {
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

    hands.sort_unstable_by_key(|(hand, _)| *hand);

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
    use super::solution;

    #[test]
    fn test1() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        let res = solution(input);
        assert_eq!(res, 6440);
    }
}
