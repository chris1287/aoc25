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
    let (_, mut data) = parse(s).expect("valid input");
    data.entry("out").or_insert_with(Vec::new);
    count_paths(
        ("svr", false, false),
        |&(key, dac, fft)| {
            data[key]
                .iter()
                .copied()
                .map(move |node| (node, dac || node == "dac", fft || node == "fft"))
        },
        |&(node, dac, fft)| node == "out" && dac && fft,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let data = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";
        assert_eq!(solve(data), 2);
    }

    #[test]
    fn test2() {
        let data = std::fs::read_to_string("input/input.txt").unwrap();
        assert_eq!(solve(&data), 290219757077250);
    }
}
