use glam::IVec2;
use nom::{
    character::complete::{newline, one_of},
    combinator::map,
    multi::{many1, separated_list1},
    IResult,
};
use nom_locate::LocatedSpan;
use std::collections::HashSet;

type Span<'a> = LocatedSpan<&'a str>;

fn parse_cell(s: Span) -> IResult<Span, Option<IVec2>> {
    let x = s.location_line() as i32 - 1;
    let y = s.get_column() as i32 - 1;
    let (s, c) = map(one_of(".S^"), |symbol| matches!(symbol, '^'))(s)?;
    let position = if c { Some(IVec2::new(x, y)) } else { None };
    Ok((s, position))
}

fn parse(s: Span) -> IResult<Span, HashSet<IVec2>> {
    let (s, map) = separated_list1(newline, many1(parse_cell))(s)?;
    Ok((
        s,
        map.into_iter()
            .flatten() // keep only Some<IVec2>
            .flatten() // flatten in a single vec
            .collect(),
    ))
}

pub fn solve(s: &str) -> usize {
    let start = IVec2::new(
        0,
        s.lines().next().expect("input should be valid").len() as i32 / 2,
    );
    let length = s.lines().count();
    let (_, splitters) = parse(Span::new(s)).expect("input should be valid");

    let mut positions = HashSet::new();
    positions.insert(start);

    let mut splits = 0;

    for _ in 0..length {
        let mut new_positions = HashSet::new();
        for pos in &positions {
            let pos = pos + IVec2::X;
            if splitters.contains(&pos) {
                // assumption: no contiguous splitters
                splits += 1;
                let l = pos + IVec2::NEG_Y;
                let r = pos + IVec2::Y;
                new_positions.insert(l);
                new_positions.insert(r);
            } else {
                // no splitter, move straight down
                new_positions.insert(pos);
            }
        }
        positions = new_positions;
    }

    splits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let data = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";
        assert_eq!(solve(data), 21);
    }

    #[test]
    fn test2() {
        let data = std::fs::read_to_string("input/input.txt").unwrap();
        assert_eq!(solve(&data.trim()), 1516);
    }
}
