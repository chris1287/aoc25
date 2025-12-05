use std::ops::RangeInclusive;

use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    combinator::all_consuming,
    multi::{many1, separated_list1},
    IResult,
};

fn parse_range(s: &str) -> IResult<&str, RangeInclusive<u64>> {
    let (s, a) = complete::u64(s)?;
    let (s, _) = tag("-")(s)?;
    let (s, b) = complete::u64(s)?;
    Ok((s, a..=b))
}

fn parse(s: &str) -> IResult<&str, (Vec<RangeInclusive<u64>>, Vec<u64>)> {
    let (s, ranges) = separated_list1(newline, parse_range)(s)?;
    let (s, _) = many1(newline)(s)?;
    let (s, ingredients) = separated_list1(newline, complete::u64)(s)?;

    Ok((s, (ranges, ingredients)))
}

fn overlaps(a: &RangeInclusive<u64>, b: &RangeInclusive<u64>) -> bool {
    a.start() <= b.end() && b.start() <= a.end() || b.start() <= a.end() && a.start() <= b.end()
}

fn merge(a: &RangeInclusive<u64>, b: &RangeInclusive<u64>) -> RangeInclusive<u64> {
    let s = std::cmp::min(a.start(), b.start());
    let e = std::cmp::max(a.end(), b.end());
    *s..=*e
}

fn step(
    range_a: &RangeInclusive<u64>,
    ranges: Vec<RangeInclusive<u64>>,
) -> Vec<RangeInclusive<u64>> {
    let mut new_ranges = Vec::<RangeInclusive<u64>>::new();
    let mut ranges_to_skip = Vec::<RangeInclusive<u64>>::new();

    for range_b in &ranges {
        if range_a == range_b {
            continue;
        }
        if overlaps(range_a, range_b) {
            new_ranges.push(merge(range_a, range_b));
            ranges_to_skip.push(range_a.clone());
            ranges_to_skip.push(range_b.clone());
            break;
        }
    }

    for range in &ranges {
        if !ranges_to_skip.contains(range) && !new_ranges.contains(range) {
            new_ranges.push(range.clone());
        }
    }

    new_ranges
}

pub fn solve(s: &str) -> u64 {
    let (_, (ranges, _ingredients)) = all_consuming(parse)(s).expect("input should be valid");
    let mut new_ranges = ranges.clone();
    loop {
        let a = new_ranges.len();
        for range_a in &new_ranges.clone() {
            new_ranges = step(range_a, new_ranges);
        }
        if a == new_ranges.len() {
            break;
        }
    }
    new_ranges
        .iter()
        .map(|range| range.end() - range.start() + 1)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test1() {
        let data = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        assert_eq!(solve(data), 14);
    }

    #[rstest]
    #[case(3, 7, 8, 10, false)]
    #[case(3, 7, 7, 10, true)]
    #[case(3, 7, 6, 10, true)]
    #[case(3, 7, 3, 7, true)]
    #[case(3, 7, 2, 3, true)]
    #[case(3, 7, 2, 4, true)]
    #[case(3, 7, 1, 2, false)]
    fn test_overlap(
        #[case] sa: u64,
        #[case] ea: u64,
        #[case] sb: u64,
        #[case] eb: u64,
        #[case] expected: bool,
    ) {
        assert_eq!(overlaps(&(sa..=ea), &(sb..=eb)), expected);
    }

    #[test]
    fn test2() {
        let data = std::fs::read_to_string("input/input.txt").unwrap();
        assert_eq!(solve(&data.trim()), 367899984917516);
    }
}
