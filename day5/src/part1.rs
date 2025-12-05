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

pub fn solve(s: &str) -> usize {
    let (_, (ranges, ingredients)) = all_consuming(parse)(s).expect("input should be valid");
    ingredients
        .iter()
        .filter(|ingredient| ranges.iter().any(|range| range.contains(ingredient)))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(solve(data), 3);
    }

    #[test]
    fn test2() {
        let data = std::fs::read_to_string("input/input.txt").unwrap();
        assert_eq!(solve(&data.trim()), 601);
    }
}
