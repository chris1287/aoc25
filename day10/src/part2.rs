use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, one_of, space1},
    combinator::map,
    multi::{many1, separated_list0, separated_list1},
    sequence::delimited,
    IResult,
};
use z3::ast::Int;
use z3::Optimize;

type Indicator = bool;
type Joltage = Vec<i64>;

#[derive(Debug)]
pub struct Button {
    pub constant: Int,
    pub indices: Vec<usize>,
    pub name: String,
}

fn parse_line(s: &str) -> IResult<&str, (Vec<Indicator>, Vec<Button>, Joltage)> {
    let (s, indicators) = delimited(
        tag("["),
        many1(map(one_of(".#"), |c| matches!(c, '#'))),
        tag("]"),
    )(s)?;

    let (s, _) = space1(s)?;

    let (s, all_indices) = separated_list1(
        space1,
        delimited(tag("("), separated_list0(tag(","), complete::u64), tag(")")),
    )(s)?;

    let mut buttons = Vec::new();
    for indices in all_indices {
        let name = format!("{:?}", indices);
        let constant = Int::new_const(name.clone());
        buttons.push(Button {
            constant,
            indices: indices.into_iter().map(|x| x as usize).collect(),
            name,
        });
    }

    let (s, _) = space1(s)?;

    let (s, joltages) = delimited(tag("{"), separated_list0(tag(","), complete::i64), tag("}"))(s)?;

    Ok((s, (indicators, buttons, joltages)))
}

fn parse(s: &str) -> IResult<&str, Vec<(Vec<Indicator>, Vec<Button>, Joltage)>> {
    let (s, v) = separated_list1(line_ending, parse_line)(s)?;
    Ok((s, v))
}

pub fn solve(s: &str) -> i64 {
    let (_, machines) = parse(s).expect("valid input");
    machines
        .into_iter()
        .map(|(_, buttons, joltages)| {
            let optimizer = Optimize::new();
            let sum = buttons.iter().fold(Int::from_i64(0), |acc, button| {
                optimizer.assert(&button.constant.ge(0));
                acc + &button.constant
            });

            for (index, &joltage) in joltages.iter().enumerate() {
                let joltage_constraint = buttons
                    .iter()
                    .filter(|button| button.indices.contains(&index))
                    .fold(Int::from_i64(0), |acc, button| acc + &button.constant);
                optimizer.assert(&joltage_constraint.eq(Int::from_i64(joltage)));
            }

            optimizer.minimize(&sum);
            match optimizer.check(&[]) {
                z3::SatResult::Sat => {
                    let model = optimizer.get_model().expect("model exists");
                    model
                        .eval(&sum, true)
                        .expect("valid value")
                        .as_i64()
                        .expect("valid i64")
                }
                z3::SatResult::Unsat => panic!("not satisfiable"),
                z3::SatResult::Unknown => panic!("unknown"),
            }
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
        assert_eq!(solve(data), 33);
    }

    #[test]
    fn test2() {
        let data = std::fs::read_to_string("input/input.txt").unwrap();
        assert_eq!(solve(&data), 21111);
    }
}
