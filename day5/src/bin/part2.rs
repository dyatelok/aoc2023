fn split_on(line: &str, mid: usize) -> (&str, &str) {
    (&line[..mid], &line[mid + 1..])
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Range {
    start: i64,
    end: i64,
}

impl Range {
    fn from(start: i64, end: i64) -> Self {
        Self { start, end }
    }
    fn contains(&self, i: i64) -> bool {
        (self.start..=self.end).contains(&i)
    }
    // if vectors intersect this functions turnes them first range regions that don't intersect
    fn split(&self, other: Self) -> Vec<Range> {
        if (self.end < other.start || other.end < self.start)
            || (other.contains(self.start) && other.contains(self.end))
        {
            // don't intersect or
            // other includes self
            //    [  s  ]
            //  [    o     ]
            //    [     ]
            vec![*self]
        } else if other.contains(self.start) {
            //     [   s   ]
            //  [    o   ]
            //     [     ][]
            vec![
                Range::from(self.start, other.end),
                Range::from(other.end + 1, self.end),
            ]
        } else if other.contains(self.end) {
            //  [   s   ]
            //      [    o   ]
            //  [  ][   ]
            vec![
                Range::from(self.start, other.start - 1),
                Range::from(other.start, self.end),
            ]
        } else {
            //  [     s    ]
            //     [  o ]
            //  [ ][    ][ ]
            vec![
                Range::from(self.start, other.start - 1),
                other,
                Range::from(other.end + 1, self.end),
            ]
        }
    }
    fn transform(&self, map: &RangeMap) -> Vec<Range> {
        self.split(map.from)
            .into_iter()
            .map(|range| {
                if range.end < map.from.start || map.from.end < range.start {
                    //if they don't intersect just return range
                    range
                } else {
                    // transform according to the map
                    let diff = map.to.start - map.from.start;
                    Self::from(range.start + diff, range.end + diff)
                }
            })
            .collect_vec()
    }
}

#[derive(Clone, Copy)]
struct RangeMap {
    from: Range,
    to: Range,
}

impl RangeMap {
    fn from((to, from, len): (i64, i64, i64)) -> Self {
        Self {
            from: Range::from(from, from + len),
            to: Range::from(to, to + len),
        }
    }
}

fn apply(ranges: Vec<Range>, range_maps: &Vec<RangeMap>) -> Vec<Range> {
    ranges
        .into_iter()
        .flat_map(|range| {
            range_maps
                .iter()
                .flat_map(|range_map| range.transform(range_map))
                .collect_vec()
        })
        .collect_vec()
}

use itertools::*;

fn get_ranges(input: &str) -> Vec<Range> {
    input
        .split_whitespace()
        .map(|num| num.parse::<i64>().unwrap())
        .tuples::<(_, _)>()
        .map(|(start, len)| Range::from(start, start + len))
        .collect()
}

fn get_range_maps(input: &str) -> Vec<RangeMap> {
    let (_, ranges) = split_on(input, input.find(':').unwrap());

    ranges
        .split_whitespace()
        .map(|num| num.parse::<i64>().unwrap())
        .tuples::<(_, _, _)>()
        .map(RangeMap::from)
        .collect()
}

fn solution(input: &str) -> i64 {
    let mut categories = input.split("\n\n");

    let seeds = categories.next().unwrap();
    let (_, seeds) = split_on(seeds, seeds.find(':').unwrap());
    let seed_ranges = get_ranges(seeds);

    let output_ranges = categories
        .map(get_range_maps)
        .fold(seed_ranges, |ranges, maps| apply(ranges, &maps));

    output_ranges
        .into_iter()
        .fold(i64::MAX, |min, range| min.min(range.start))
}

fn main() {
    let input = include_str!("../../input1.txt");
    let res = solution(input);
    println!("{res}");
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use crate::{apply, get_range_maps, get_ranges, split_on};
    use crate::{Range, RangeMap};

    use super::solution;

    #[test]
    fn test_solution() {
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
        assert_eq!(res, 46);
    }
    #[test]
    fn test_split_no_itersection() {
        let range1 = Range::from(20, 40);
        let range2 = Range::from(45, 60);

        assert_eq!(range1.split(range2), vec![range1]);

        let range1 = Range::from(45, 60);
        let range2 = Range::from(20, 40);

        assert_eq!(range1.split(range2), vec![range1]);
    }
    #[test]
    fn test_inside() {
        let range1 = Range::from(20, 40);
        let range2 = Range::from(10, 60);

        assert_eq!(range1.split(range2), vec![range1]);
    }
    #[test]
    fn test_contains_start() {
        let range1 = Range::from(20, 40);
        let range2 = Range::from(30, 60);

        assert_eq!(
            range1.split(range2),
            vec![Range::from(20, 29), Range::from(30, 40)]
        );
    }
    #[test]
    fn test_contains_end() {
        let range1 = Range::from(30, 60);
        let range2 = Range::from(20, 40);

        assert_eq!(
            range1.split(range2),
            vec![Range::from(30, 40), Range::from(41, 60)]
        );
    }
    #[test]
    fn test_contains_both() {
        let range1 = Range::from(10, 60);
        let range2 = Range::from(20, 40);

        assert_eq!(
            range1.split(range2),
            vec![
                Range::from(10, 19),
                Range::from(20, 40),
                Range::from(41, 60)
            ]
        );
    }
    #[test]
    fn test_split_transform() {
        let range = Range::from(20, 60);
        let map = RangeMap::from((21, 40, 15));

        assert_eq!(
            range.split(map.from),
            vec![
                Range::from(20, 39),
                Range::from(40, 55),
                Range::from(56, 60),
            ]
        );

        assert_eq!(
            range.transform(&map),
            vec![
                Range::from(20, 39),
                Range::from(21, 36),
                Range::from(56, 60),
            ]
        );
    }
    #[test]
    fn test_apply() {
        let ranges = vec![Range::from(79, 93), Range::from(55, 68)];
        let maps = vec![RangeMap::from((50, 98, 2)), RangeMap::from((52, 50, 48))];

        assert_eq!(ranges[0].split(maps[0].from), vec![Range::from(79, 93)]);
        assert_eq!(ranges[0].split(maps[1].from), vec![Range::from(79, 93)]);
        assert_eq!(ranges[1].split(maps[0].from), vec![Range::from(55, 68)]);
        assert_eq!(ranges[1].split(maps[1].from), vec![Range::from(55, 68)]);

        assert_eq!(ranges[0].transform(&maps[0]), vec![Range::from(79, 93)]);
        assert_eq!(ranges[0].transform(&maps[1]), vec![Range::from(81, 95)]);
        assert_eq!(ranges[1].transform(&maps[0]), vec![Range::from(55, 68)]);
        assert_eq!(ranges[1].transform(&maps[1]), vec![Range::from(57, 70)]);

        assert_eq!(
            apply(ranges, &maps),
            vec![
                Range::from(79, 93),
                Range::from(81, 95),
                Range::from(55, 68),
                Range::from(57, 70),
            ]
        );
    }
    #[test]
    fn test_real() {
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
        let mut categories = input.split("\n\n");

        let seeds = categories.next().unwrap();
        let (_, seeds) = split_on(seeds, seeds.find(':').unwrap());
        let seed_ranges = get_ranges(seeds);

        let range_maps = categories.map(get_range_maps).collect_vec();

        assert_eq!(
            apply(seed_ranges, &range_maps[0]),
            vec![
                Range::from(79, 93),
                Range::from(81, 95),
                Range::from(55, 68),
                Range::from(57, 70),
            ]
        );
    }
}
