use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::separated_list1,
    IResult,
};

use geo::*;

fn parse(s: &str) -> IResult<&str, Vec<Coord>> {
    let (s, v) = separated_list1(line_ending, separated_list1(tag(","), complete::i64))(s)?;
    let v = v
        .into_iter()
        .map(|tmp| coord!(x: tmp[0] as f64, y: tmp[1] as f64))
        .collect::<Vec<Coord>>();
    Ok((s, v))
}

pub fn solve(s: &str) -> i64 {
    let (_, coordinates) = parse(s).expect("valid input");

    let poly = Polygon::new(coordinates.into(), vec![]);

    let (p1, p2, max) = poly
        .exterior_coords_iter()
        .cartesian_product(poly.exterior_coords_iter())
        .map(|(p1, p2)| {
            let b = (p1.x.max(p2.x) - p1.x.min(p2.x) + 1.0).abs();
            let h = (p1.y.max(p2.y) - p1.y.min(p2.y) + 1.0).abs();
            let area = (b * h) as i64;
            (p1, p2, area)
        })
        .sorted_by_key(|(_p1, _p2, area)| *area)
        .rev()
        .find(|(p1, p2, _)| {
            let top_left = coord!(x: p1.x.min(p2.x), y: p1.y.min(p2.y));
            let top_right = coord!(x: p1.x.max(p2.x), y: p1.y.min(p2.y));
            let bot_left = coord!(x: p1.x.min(p2.x), y: p1.y.max(p2.y));
            let bot_right = coord!(x: p1.x.max(p2.x), y: p2.y.max(p1.y));
            let rect = Polygon::new(
                vec![top_left, top_right, bot_right, bot_left, top_left].into(),
                vec![],
            );
            if poly.contains(&rect) {
                return true;
            }
            false
        })
        .expect("solution");

    dbg!(&p1, &p2, &max);
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
        assert_eq!(solve(data), 24);
    }

    #[test]
    fn test2() {
        let data = std::fs::read_to_string("input/input.txt").unwrap();
        assert_eq!(solve(&data), 1588990708);
    }
}
