use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, one_of, space1},
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult,
};

#[derive(Debug)]
pub struct Region {
    pub w: u32,
    pub h: u32,
    pub shapes: Vec<u32>,
}

trait Area {
    fn area(&self) -> u32;
}

impl Area for Vec<bool> {
    fn area(&self) -> u32 {
        self.iter().filter(|&&x| x).count() as u32
    }
}

fn parse_shape(s: &str) -> IResult<&str, Vec<bool>> {
    let (s, _) = complete::u32(s)?;
    let (s, _) = tag(":")(s)?;
    let (s, _) = line_ending(s)?;
    let (s, c) = separated_list1(line_ending, many1(one_of("#.")))(s)?;
    let c = c
        .into_iter()
        .flatten()
        .map(|x| matches!(x, '#'))
        .collect::<Vec<bool>>();
    Ok((s, c))
}

fn parse_shapes(s: &str) -> IResult<&str, Vec<Vec<bool>>> {
    separated_list1(many1(line_ending), parse_shape)(s)
}

fn parse_size(s: &str) -> IResult<&str, (u32, u32)> {
    let (s, (w, h)) = separated_pair(complete::u32, tag("x"), complete::u32)(s)?;
    let (s, _) = tag(":")(s)?;
    Ok((s, (w, h)))
}

fn parse_region(s: &str) -> IResult<&str, Region> {
    let (s, ((w, h), shapes)) =
        separated_pair(parse_size, space1, separated_list1(space1, complete::u32))(s)?;
    Ok((s, Region { w, h, shapes }))
}

fn parse_regions(s: &str) -> IResult<&str, Vec<Region>> {
    separated_list1(line_ending, parse_region)(s)
}

fn parse(s: &str) -> IResult<&str, (Vec<Vec<bool>>, Vec<Region>)> {
    let (s, shapes) = parse_shapes(s)?;
    let (s, _) = many1(line_ending)(s)?;
    let (s, regions) = parse_regions(s)?;
    Ok((s, (shapes, regions)))
}

pub fn solve(s: &str) -> usize {
    let (_, (shapes, regions)) = parse(s).expect("valid input");

    regions
        .into_iter()
        .filter(|region| {
            let available_area = region.w * region.h;
            let required_area = shapes
                .iter()
                .enumerate()
                .map(|(idx, shape)| region.shapes[idx] * shape.area())
                .sum::<u32>();
            available_area >= required_area
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test2() {
        let data = std::fs::read_to_string("input/input.txt").unwrap();
        assert_eq!(solve(&data), 531);
    }
}
