use std::cmp::{max, min};

use glam::IVec2;
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::separated_list1,
    IResult,
};

fn parse(s: &str) -> IResult<&str, Vec<IVec2>> {
    let (s, v) = separated_list1(line_ending, separated_list1(tag(","), complete::i32))(s)?;
    Ok((
        s,
        v.iter()
            .map(|tmp| IVec2::new(tmp[0], tmp[1]))
            .collect::<Vec<IVec2>>(),
    ))
}

pub fn solve(s: &str) -> i64 {
    let (_, data) = parse(s).expect("valid input");
    let (_p1, _p2, max) = data
        .iter()
        .cartesian_product(&data)
        .filter_map(|(p1, p2)| {
            if p1 == p2 {
                return None;
            }
            let b = (max(p1.x, p2.x) - min(p1.x, p2.x) + 1) as i64;
            let h = (max(p1.y, p2.y) - min(p1.y, p2.y) + 1) as i64;
            if b == 0 || h == 0 {
                return None;
            }
            Some((p1, p2, b * h))
        })
        .max_by_key(|(_p1, _p2, area)| *area)
        .unwrap();
    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let data = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
        assert_eq!(solve(data), 50);
    }

    #[test]
    fn test2() {
        let data = std::fs::read_to_string("input/input.txt").unwrap();
        assert_eq!(solve(&data), 4769758290);
    }
}
