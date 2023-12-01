#![feature(lazy_cell)]
use std::sync::LazyLock;

static DIGITS: LazyLock<Vec<(&str, u32)>> = LazyLock::new(|| {
    vec![
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]
});

fn transform(vec: Vec<(Option<usize>, u32)>) -> impl Iterator<Item = (usize, u32)> {
    vec.into_iter()
        .filter_map(|(pos, value)| pos.map(|pos| (pos, value)))
}

fn process_line(line: &str) -> u32 {
    let (first_match, last_match): (Vec<_>, Vec<_>) = DIGITS
        .iter()
        .map(|(digit, value)| ((line.find(digit), *value), (line.rfind(digit), *value)))
        .unzip();

    let first_lit = transform(first_match).min();
    let last_lit = transform(last_match).max();

    let vec: Vec<(usize, u32)> = line
        .chars()
        .enumerate()
        .filter_map(|(pos, value)| value.to_digit(10).map(|value| (pos, value)))
        .collect();
    let (first_dig, last_dig) = (vec.first().copied(), vec.last().copied());

    let first = [first_lit, first_dig].iter().flatten().min().unwrap().1;
    let last = [last_lit, last_dig].iter().flatten().max().unwrap().1;

    first * 10 + last
}

fn main() {
    let input = include_str!("../../input1.txt");
    let res = input.lines().map(process_line).sum::<u32>();
    println!("{res}");
}

#[cfg(test)]
mod tests {
    use super::process_line;
    #[test]
    fn exploration() {
        let line = "two1";
        let res = process_line(line);
        println!("{res}");
    }
}
