use glam::IVec2;
use nom::{
    character::complete::{newline, one_of},
    combinator::map,
    multi::{many1, separated_list1},
    IResult,
};
use nom_locate::LocatedSpan;
use std::collections::{HashMap, HashSet};

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

fn step(
    depth: i32,
    pos: &IVec2,
    splitters: &HashSet<IVec2>,
    cache: &mut HashMap<IVec2, usize>,
) -> usize {
    if pos.x == depth {
        return 1;
    }
    if splitters.contains(pos) {
        // assumption: no contiguous splitters
        let l = pos + IVec2::NEG_Y;
        let r = pos + IVec2::Y;
        let count_left = if let Some(tmp) = cache.get(&l) {
            *tmp
        } else {
            let tmp = step(depth, &l, splitters, cache);
            cache.insert(l, tmp);
            tmp
        };
        let count_right = if let Some(tmp) = cache.get(&r) {
            *tmp
        } else {
            let tmp = step(depth, &r, splitters, cache);
            cache.insert(r, tmp);
            tmp
        };
        count_left + count_right
    } else {
        // no splitter, move straight down
        let tmp = step(depth, &(pos + IVec2::X), splitters, cache);
        cache.insert(pos + IVec2::X, tmp);
        tmp
    }
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

    let mut cache = HashMap::new();
    step(length as i32, &start, &splitters, &mut cache)
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
        assert_eq!(solve(data), 40);
    }

    #[test]
    fn test2() {
        let data = std::fs::read_to_string("input/input.txt").unwrap();
        assert_eq!(solve(&data.trim()), 1393669447690);
    }
}
