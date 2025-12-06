use nom::{
    character::complete::{self, newline, one_of, space0, space1},
    combinator::map,
    multi::{many1, separated_list1},
    IResult,
};

#[derive(Debug)]
enum Operator {
    Sum,
    Multiply,
}

fn parse_numbers(s: &str) -> IResult<&str, Vec<u64>> {
    let (s, _) = space0(s)?;
    let (s, numbers) = separated_list1(space1, complete::u64)(s)?;
    let (s, _) = space0(s)?;
    Ok((s, numbers))
}

fn parse_operators(s: &str) -> IResult<&str, Vec<Operator>> {
    let (s, _) = space0(s)?;
    let (s, operators) = separated_list1(
        space1,
        map(one_of("+*"), |c| match c {
            '+' => Operator::Sum,
            '*' => Operator::Multiply,
            _ => panic!("invalid operator"),
        }),
    )(s)?;
    let (s, _) = space0(s)?;
    Ok((s, operators))
}

fn parse(s: &str) -> IResult<&str, (Vec<Vec<u64>>, Vec<Operator>)> {
    let (s, numbers) = separated_list1(newline, parse_numbers)(s)?;
    let (s, _) = many1(newline)(s)?;
    let (s, operators) = parse_operators(s)?;
    Ok((s, (numbers, operators)))
}

pub fn solve(s: &str) -> u64 {
    let (_, (numbers, operators)) = parse(s).expect("valid data");

    let mut total = 0;
    for col in 0..operators.len() {
        let mut res = match operators[col] {
            Operator::Sum => 0,
            Operator::Multiply => 1,
        };
        for series in &numbers {
            match operators[col] {
                Operator::Sum => res += series[col],
                Operator::Multiply => res *= series[col],
            };
        }
        total += res;
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let data = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
        assert_eq!(solve(data), 4277556);
    }

    #[test]
    fn test2() {
        let data = std::fs::read_to_string("input/input.txt").unwrap();
        assert_eq!(solve(&data.trim()), 6378679666679);
    }
}
