use std::collections::HashMap;

use glam::IVec2;
use nom::{
    character::complete::{line_ending, one_of},
    multi::{many1, separated_list1},
    IResult,
};

use nom_locate::LocatedSpan;

type Span<'a> = LocatedSpan<&'a str>;

const DIRECTIONS: [IVec2; 8] = [
    IVec2::new(1, 0),
    IVec2::new(-1, 0),
    IVec2::new(0, 1),
    IVec2::new(0, -1),
    IVec2::new(1, 1),
    IVec2::new(1, -1),
    IVec2::new(-1, 1),
    IVec2::new(-1, -1),
];

fn parse_cell(s: Span) -> IResult<Span, (IVec2, bool)> {
    let x = s.location_line() as i32 - 1;
    let y = s.get_column() as i32 - 1;
    let (s, c) = one_of("@.")(s)?;
    let c = matches!(c, '@');
    Ok((s, (IVec2::new(x, y), c)))
}

fn parse(s: Span) -> IResult<Span, HashMap<IVec2, bool>> {
    let (s, v) = separated_list1(line_ending, many1(parse_cell))(s)?;
    Ok((s, v.into_iter().flatten().collect::<HashMap<IVec2, bool>>()))
}

fn step(data: &mut HashMap<IVec2, bool>) -> bool {
    let old_data = data.clone();
    data.retain(|cell, value| {
        if !*value {
            return true;
        }
        let count = DIRECTIONS.iter().fold(0, |acc, dir| {
            if let Some(content) = old_data.get(&(cell + dir)) {
                if *content {
                    return acc + 1;
                }
            }
            acc
        });
        count >= 4
    });

    old_data.len() != data.len()
}

pub fn solve(s: &str) -> usize {
    let (_, mut v) = parse(Span::new(s)).expect("input should be valid");

    let cells = v.len();
    while step(&mut v) {}

    cells - v.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let data = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        assert_eq!(solve(data), 43);
    }

    #[test]
    fn test2() {
        let data = std::fs::read_to_string("input/input.txt").unwrap();
        assert_eq!(solve(&data.trim()), 8442);
    }
}
