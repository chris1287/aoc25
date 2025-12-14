use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, line_ending, space1},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

use std::collections::HashMap;

use pathfinding::prelude::count_paths;

fn parse_line(s: &str) -> IResult<&str, (&str, Vec<&str>)> {
    let (s, (k, v)) = separated_pair(alpha1, tag(": "), separated_list1(space1, alpha1))(s)?;
    Ok((s, (k, v)))
}

fn parse(s: &str) -> IResult<&str, HashMap<&str, Vec<&str>>> {
    let (s, data) = separated_list1(line_ending, parse_line)(s)?;
    let data = data.into_iter().collect::<HashMap<_, _>>();
    Ok((s, data))
}

pub fn solve(s: &str) -> usize {
    let (_, data) = parse(s).expect("valid input");
    count_paths(
        "you",
        |key| data[key].iter().copied(),
        |node| *node == "out",
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let data = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";
        assert_eq!(solve(data), 5);
    }

    #[test]
    fn test2() {
        let data = std::fs::read_to_string("input/input.txt").unwrap();
        assert_eq!(solve(&data), 733);
    }
}
