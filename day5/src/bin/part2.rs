fn split_on(line: &str, mid: usize) -> (&str, &str) {
    (&line[..mid], &line[mid + 1..])
}

#[derive(Clone, Copy)]
struct Range {
    start: usize,
    end: usize,
}

impl Range {
    fn from(start: usize, end: usize) -> Self {
        Self { start, end }
    }
    fn contains(&self, i: usize) -> bool {
        (self.start..=self.end).contains(&i)
    }
    fn intersects(&self, other: Self) -> bool {
        if self.contains(other.start)
            || self.contains(other.end)
            || other.contains(self.start)
            || other.contains(self.end)
        {
            true
        } else {
            false
        }
    }
}

struct RangeMap {
    from: Range,
    to: Range,
}

impl RangeMap {
    fn from((to, from, len): (usize, usize, usize)) -> Self {
        Self {
            from: Range::from(from, from + len),
            to: Range::from(to, to + len),
        }
    }
}

use itertools::*;

fn get_ranges(input: &str) -> Vec<Range> {
    input
        .split_whitespace()
        .map(|num| num.parse::<usize>().unwrap())
        .tuples::<(_, _)>()
        .map(|(start, end)| Range::from(start, end))
        .collect()
}

fn get_range_maps(input: &str) -> Vec<RangeMap> {
    let (_, ranges) = split_on(input, input.find(':').unwrap());

    ranges
        .split_whitespace()
        .map(|num| num.parse::<usize>().unwrap())
        .tuples::<(_, _, _)>()
        .map(RangeMap::from)
        .collect()
}

fn apply(mut ranges: Vec<Range>, mut range_maps: Vec<RangeMap>) -> Vec<Range> {
    ranges.sort_unstable_by(|a, b| a.start.cmp(&b.start));
    range_maps.sort_unstable_by(|a, b| a.from.start.cmp(&b.from.start));

    let mut result = Vec::new();

    let (mut i, mut j) = (0, 0);

    while i != ranges.len() && j != range_maps.len() {
        if ranges[i].intersects(range_maps[j].from) {
            if range_maps[j].from.contains(ranges[i].start) && range_maps[j].from.contains(ranges[i].end) {
                //     [ inp ]
                //  [  matching ]
                result.push(ranges[i]);
            } else if range_maps[j].from.contains(ranges[i].start) {
                //      [  inp  ]
                //   [ maches ]
                // TODO
                result.push(Range::from(, ));
                
            }
        } else if ranges[i].start > range_maps[j].from.start {
            if j != range_maps.len() {
                j += 1;
            } else {
                i += 1;
            }
        } else {
            if i != ranges.len() {
                i += 1;
            } else {
                j += 1;
            }
        }
    }

    result
}

fn solution(input: &str) -> usize {
    let mut categories = input.split("\n\n");

    let seeds = categories.next().unwrap();
    let (_, seeds) = split_on(seeds, seeds.find(':').unwrap());
    let seed_ranges = get_ranges(seeds);

    let output_ranges = categories.map(get_range_maps).fold(seed_ranges, apply);

    0
}

fn main() {
    let input = include_str!("../../input1.txt");
    let res = solution(input);
    println!("{res}");
}

// #[cfg(test)]
// mod tests {
//     use super::solution;

//     #[test]
//     fn test1() {
//         let input = "seeds: 79 14 55 13

// seed-to-soil map:
// 50 98 2
// 52 50 48

// soil-to-fertilizer map:
// 0 15 37
// 37 52 2
// 39 0 15

// fertilizer-to-water map:
// 49 53 8
// 0 11 42
// 42 0 7
// 57 7 4

// water-to-light map:
// 88 18 7
// 18 25 70

// light-to-temperature map:
// 45 77 23
// 81 45 19
// 68 64 13

// temperature-to-humidity map:
// 0 69 1
// 1 0 69

// humidity-to-location map:
// 60 56 37
// 56 93 4
// ";
//         let res = solution(input);
//         assert_eq!(res, 35);
//     }
// }
