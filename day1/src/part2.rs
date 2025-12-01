use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{i32, line_ending},
    combinator::map,
    multi::separated_list1,
    IResult,
};

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct Rotation {
    direction: Direction,
    steps: i32,
}

fn parse_line(s: &str) -> IResult<&str, Rotation> {
    let (s, direction) = alt((
        map(tag("L"), |_| Direction::Left),
        map(tag("R"), |_| Direction::Right),
    ))(s)?;
    let (s, steps) = i32(s)?;
    Ok((s, Rotation { direction, steps }))
}

fn parse(s: &str) -> IResult<&str, Vec<Rotation>> {
    let (s, v) = separated_list1(line_ending, parse_line)(s)?;
    Ok((s, v))
}

pub fn solve(s: &str) -> i32 {
    let (_, v) = parse(s).expect("input should be valid");
    let mut pos: i32 = 50;
    v.iter()
        .fold(0, |acc, r| {
            let mut cross = 0;
            for _ in 0..r.steps {
                match r.direction {
                    Direction::Left  => pos = (pos - 1).rem_euclid(100),
                    Direction::Right => pos = (pos + 1).rem_euclid(100)
                };
                if pos == 0 {
                    cross += 1
                }
            }

            cross + acc
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let data = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        assert_eq!(solve(data), 6);
    }

    #[test]
    fn test2() {
        let data = std::fs::read_to_string("input/input.txt").unwrap();
        assert_eq!(solve(&data), 6558);
    }
}
