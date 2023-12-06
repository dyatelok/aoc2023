fn split_on(line: &str, mid: usize) -> (&str, &str) {
    (&line[..mid], &line[mid + 1..])
}

struct Map {
    ranges: Vec<(usize, usize, usize)>,
}

impl Map {
    fn new() -> Self {
        Self { ranges: vec![] }
    }
    fn add_range(&mut self, range: (usize, usize, usize)) {
        self.ranges.push(range);
    }
    fn apply(&self, n: usize) -> usize {
        for (to, from, len) in &self.ranges {
            if (*from..*from + *len).contains(&n) {
                return n - from + to;
            }
        }
        n
    }
}

struct MapStack {
    layers: Vec<Map>,
}

impl MapStack {
    fn new() -> Self {
        Self { layers: vec![] }
    }
    fn add_map(&mut self, map: Map) {
        self.layers.push(map);
    }
    fn apply(&self, n: usize) -> usize {
        self.layers.iter().fold(n, |n, map| map.apply(n))
    }
}

use itertools::*;

fn get_data(input: &str) -> Map {
    let (_, ranges) = split_on(input, input.find(':').unwrap());

    ranges
        .split_whitespace()
        .map(|num| num.parse::<usize>().unwrap())
        .tuples::<(_, _, _)>()
        .fold(Map::new(), |mut map, range| {
            map.add_range(range);
            map
        })
}

fn solution(input: &str) -> usize {
    let mut categories = input.split("\n\n");

    let seeds = categories.next().unwrap();
    let (_, seeds) = split_on(seeds, seeds.find(':').unwrap());
    let seeds = seeds
        .split_whitespace()
        .map(|num| num.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let stack = categories
        .map(get_data)
        .fold(MapStack::new(), |mut stack, map| {
            stack.add_map(map);
            stack
        });

    seeds
        .into_iter()
        .map(|seed| stack.apply(seed))
        .min()
        .unwrap()
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
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";
        let res = solution(input);
        assert_eq!(res, 35);
    }
}
