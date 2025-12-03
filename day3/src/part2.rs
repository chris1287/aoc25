use nom::{
    character::complete::{newline, one_of},
    combinator::{all_consuming, map},
    multi::{many1, separated_list1},
    IResult,
};

fn parse(s: &str) -> IResult<&str, Vec<Vec<u64>>> {
    let (s, v) = all_consuming(separated_list1(
        newline,
        many1(map(one_of("0123456789"), |c| {
            c.to_digit(10).expect("invalid input") as u64
        })),
    ))(s)?;

    Ok((s, v))
}

fn mymax(skip: usize, v: &[u64]) -> (usize, u64) {
    let mut m = &v[skip];
    let mut idx = skip;
    for (i, num) in v.iter().enumerate().skip(skip) {
        if num > m {
            m = num;
            idx = i;
        }
    }
    (idx, *m)
}

pub fn solve(s: &str) -> u64 {
    let (_, v) = parse(s).expect("input should be valid");
    v.iter()
        .map(|nums| {
            let mut res;
            let mut idx = 0;
            (idx, res) = mymax(idx, &nums[..nums.len() - 11]);
            for i in (0..11).rev() {
                let v;
                (idx, v) = mymax(idx + 1, &nums[..nums.len() - i]);
                res = res * 10 + v;
            }
            res
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
        assert_eq!(solve(data), 3121910778619);
    }

    #[test]
    fn test2() {
        let data = std::fs::read_to_string("input/input.txt").unwrap();
        assert_eq!(solve(&data.trim()), 171741365473332);
    }
}
