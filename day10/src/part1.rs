use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, one_of, space1},
    combinator::map,
    multi::{many1, separated_list0, separated_list1},
    sequence::delimited,
    IResult,
};

type Indicator = bool;
type Wiring = Vec<usize>;
type Joltage = Vec<usize>;

fn parse_line(s: &str) -> IResult<&str, (Vec<Indicator>, Vec<Wiring>, Joltage)> {
    let (s, indicators) = delimited(
        tag("["),
        many1(map(one_of(".#"), |c| matches!(c, '#'))),
        tag("]"),
    )(s)?;

    let (s, _) = space1(s)?;

    let (s, buttons) = separated_list1(
        space1,
        delimited(tag("("), separated_list0(tag(","), complete::u64), tag(")")),
    )(s)?;

    let buttons = buttons
        .into_iter()
        .map(|v| v.into_iter().map(|x| x as usize).collect())
        .collect();

    let (s, _) = space1(s)?;

    let (s, joltages) = delimited(tag("{"), separated_list0(tag(","), complete::u64), tag("}"))(s)?;

    let joltages = joltages.into_iter().map(|x| x as usize).collect();

    Ok((s, (indicators, buttons, joltages)))
}

fn parse(s: &str) -> IResult<&str, Vec<(Vec<Indicator>, Vec<Wiring>, Joltage)>> {
    let (s, v) = separated_list1(line_ending, parse_line)(s)?;
    Ok((s, v))
}

fn solved(actual: &[Indicator], desired: &[Indicator]) -> bool {
    actual == desired
}

fn try_solve(len: usize, wirings: &[&Wiring]) -> Vec<Indicator> {
    let mut actual = vec![false; len];
    for wiring in wirings {
        for idx in *wiring {
            actual[*idx] = !actual[*idx];
        }
    }
    actual
}

fn find_solution(len: usize, desired: &[Indicator], wirings: &[Wiring]) -> usize {
    let mut comb_len = 1;
    loop {
        let solution = wirings
            .iter()
            .sorted_by_key(|wiring| wiring.len())
            .combinations_with_replacement(comb_len)
            .find(|combs| {
                let solution = try_solve(len, combs);
                if solved(desired, &solution) {
                    return true;
                }
                false
            });
        if let Some(_solution) = solution {
            break;
        }
        comb_len += 1;
    }
    comb_len
}

pub fn solve(s: &str) -> usize {
    let (_, data) = parse(s).expect("valid input");

    data.iter()
        .map(|(desired, wirings, _)| {
            let len = desired.len();
            find_solution(len, desired, wirings)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let data = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        assert_eq!(solve(data), 7);
    }

    #[test]
    fn test2() {
        let data = std::fs::read_to_string("input/input.txt").unwrap();
        assert_eq!(solve(&data), 547);
    }
}
