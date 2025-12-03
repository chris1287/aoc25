use nom::{
    character::complete::{newline, one_of},
    combinator::{all_consuming, map},
    multi::{many1, separated_list1},
    IResult,
};

fn parse(s: &str) -> IResult<&str, Vec<Vec<u32>>> {
    let (s, v) = all_consuming(separated_list1(
        newline,
        many1(map(one_of("0123456789"), |c| {
            c.to_digit(10).expect("invalid input")
        })),
    ))(s)?;

    Ok((s, v))
}

fn mymax(v: &[u32]) -> (usize, u32) {
    let mut m = &v[0];
    let mut idx = 0;
    for (i, num) in v.iter().enumerate().skip(1) {
        if num > m {
            m = num;
            idx = i;
        }
    }
    (idx, *m)
}

pub fn solve(s: &str) -> u32 {
    let (_, v) = parse(s).expect("input should be valid");
    v.iter()
        .map(|nums| {
            let (idxa, a) = mymax(&nums[..nums.len() - 1]);
            let b = nums[idxa + 1..nums.len()]
                .iter()
                .max()
                .expect("invalid input");
            a * 10 + b
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let data = "987654321111111
811111111111119
234234234234278
818181911112111";
        assert_eq!(solve(data), 357);
    }

    #[test]
    fn test2() {
        let data = std::fs::read_to_string("input/input.txt").unwrap();
        assert_eq!(solve(&data.trim()), 17316);
    }
}
