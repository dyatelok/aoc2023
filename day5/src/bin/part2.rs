fn split_on(line: &str, mid: usize) -> (&str, &str) {
    (&line[..mid], &line[mid + 1..])
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord)]
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
                Range::from(other.start, other.end),
                Range::from(other.end + 1, self.end),
            ]
        }
    }
    fn transform(&self, map: &RangeMap) -> (Vec<Range>, Vec<Range>) {
        let mut left = vec![];
        let mut new = vec![];
        self.split(map.from).into_iter().for_each(|range| {
            if range.end < map.from.start || map.from.end < range.start {
                //if they don't intersect just return range
                left.push(range);
            } else {
                // transform according to the map
                let diff = map.to.start - map.from.start;
                new.push(Self::from(range.start + diff, range.end + diff));
            }
        });
        (left, new)
    }
}

#[derive(Clone, Copy, Debug)]
struct RangeMap {
    from: Range,
    to: Range,
}

impl RangeMap {
    fn from((to, from, len): (i64, i64, i64)) -> Self {
        Self {
            from: Range::from(from, from + len - 1),
            to: Range::from(to, to + len - 1),
        }
    }
}

fn apply(ranges: Vec<Range>, range_maps: &Vec<RangeMap>) -> Vec<Range> {
    let mut left_ranges = ranges;
    let mut new_ranges = vec![];

    for map in range_maps {
        left_ranges = left_ranges
            .into_iter()
            .flat_map(|range| {
                let (left, transformed) = range.transform(map);
                new_ranges.extend(transformed.into_iter());
                left
            })
            .collect();
    }
    left_ranges.extend(new_ranges.into_iter());
    left_ranges.sort_unstable();
    left_ranges.into_iter().fold(vec![], |mut vec, range| {
        if vec.is_empty() {
            vec.push(range);
        } else {
            if vec.last().unwrap().end == range.start - 1 {
                vec.last_mut().unwrap().end = range.end;
            } else {
                vec.push(range);
            }
        }
        vec
    })
}

use itertools::*;

fn get_ranges(input: &str) -> Vec<Range> {
    input
        .split_whitespace()
        .map(|num| num.parse::<i64>().unwrap())
        .tuples::<(_, _)>()
        .map(|(start, len)| Range::from(start, start + len - 1))
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
    use crate::apply;
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
56 93 4";
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
    fn test_apply() {
        let ranges0 = vec![Range::from(55, 67), Range::from(79, 92)];
        let ranges1 = vec![Range::from(57, 69), Range::from(81, 94)];
        let ranges2 = vec![Range::from(57, 69), Range::from(81, 94)];
        let ranges3 = vec![
            Range::from(53, 56),
            Range::from(61, 69),
            Range::from(81, 94),
        ];
        let ranges4 = vec![
            Range::from(46, 49),
            Range::from(54, 62),
            Range::from(74, 87),
        ];
        let ranges5 = vec![
            Range::from(45, 55),
            Range::from(78, 80),
            Range::from(82, 85),
            Range::from(90, 98),
        ];
        let ranges6 = vec![
            Range::from(46, 56),
            Range::from(78, 80),
            Range::from(82, 85),
            Range::from(90, 98),
        ];
        let ranges7 = vec![
            Range::from(46, 60),
            Range::from(82, 84),
            Range::from(86, 89),
            Range::from(94, 98),
        ];

        let maps0 = vec![RangeMap::from((50, 98, 2)), RangeMap::from((52, 50, 48))];
        let maps1 = vec![
            RangeMap::from((0, 15, 37)),
            RangeMap::from((37, 52, 2)),
            RangeMap::from((39, 0, 15)),
        ];
        let maps2 = vec![
            RangeMap::from((49, 53, 8)),
            RangeMap::from((0, 11, 42)),
            RangeMap::from((42, 0, 7)),
            RangeMap::from((57, 7, 4)),
        ];
        let maps3 = vec![RangeMap::from((88, 18, 7)), RangeMap::from((18, 25, 70))];
        let maps4 = vec![
            RangeMap::from((45, 77, 23)),
            RangeMap::from((81, 45, 19)),
            RangeMap::from((68, 64, 13)),
        ];
        let maps5 = vec![RangeMap::from((0, 69, 1)), RangeMap::from((1, 0, 69))];
        let maps6 = vec![RangeMap::from((60, 56, 37)), RangeMap::from((56, 93, 4))];

        debug_assert_eq!(apply(ranges0, &maps0), ranges1);
        debug_assert_eq!(apply(ranges1, &maps1), ranges2);
        debug_assert_eq!(apply(ranges2, &maps2), ranges3);
        debug_assert_eq!(apply(ranges3, &maps3), ranges4);
        debug_assert_eq!(apply(ranges4, &maps4), ranges5);
        debug_assert_eq!(apply(ranges5, &maps5), ranges6);
        debug_assert_eq!(apply(ranges6, &maps6), ranges7);
    }
}
