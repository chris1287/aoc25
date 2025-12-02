use itertools::Itertools;
use nom::{
    bytes::complete::{is_not, tag},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

fn valid(s: &str) -> bool {
    let l = s.len();

    for chunk_size in 1..=l / 2 {
        if !l.is_multiple_of(chunk_size) {
            continue;
        }
        let v = &s
            .chars()
            .chunks(chunk_size)
            .into_iter()
            .map(|chunk| chunk.collect::<String>())
            .collect::<Vec<String>>();
        if v.iter().all_equal() {
            return false;
        }
    }

    true
}

fn parse(s: &str) -> IResult<&str, Vec<(&str, &str)>> {
    let (s, v) = separated_list1(
        tag(","),
        separated_pair(is_not("-,\r\n"), tag("-"), is_not("-,\r\n")),
    )(s)?;
    Ok((s, v))
}

pub fn solve(s: &str) -> usize {
    let (_, v) = parse(s).expect("input should be valid");

    let mut invalid = Vec::new();

    for (start, end) in v {
        let s: usize = start.parse().expect("start should be a valid integer");
        let e: usize = end.parse().expect("end should be a valid integer");

        for n in s..=e {
            let n = n.to_string();
            if !valid(&n) {
                invalid.push(n);
            }
        }
    }

    dbg!(&invalid);

    invalid
        .iter()
        .map(|s| s.parse::<usize>().expect("should be a valid integer"))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let data = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(solve(data), 4174379265);
    }

    #[test]
    fn test2() {
        let data = std::fs::read_to_string("input/input.txt").unwrap();
        assert_eq!(solve(&data), 65794984339);
    }
}
