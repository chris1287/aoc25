use std::collections::{BTreeMap, HashSet};

use glam::IVec3;
use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    IResult,
};

fn parse(s: &str) -> IResult<&str, Vec<IVec3>> {
    let (s, v) = separated_list1(newline, separated_list1(tag(","), complete::i32))(s)?;
    Ok((s, v.iter().map(|x| IVec3::new(x[0], x[1], x[2])).collect()))
}

fn compute_distances(data: &[IVec3]) -> BTreeMap<i64, (IVec3, IVec3)> {
    let mut map = BTreeMap::new();
    for i in 0..data.len() {
        for j in (i + 1)..data.len() {
            let a = data[i];
            let b = data[j];
            let dx = (a.x - b.x) as i64;
            let dy = (a.y - b.y) as i64;
            let dz = (a.z - b.z) as i64;
            let d = dx * dx + dy * dy + dz * dz;
            map.insert(d, (a, b));
        }
    }
    map
}

pub fn solve(s: &str) -> i32 {
    let (_, data) = parse(s).expect("valid input");

    let nodes = data.len();
    let distances = compute_distances(&data);

    let mut clusters: Vec<HashSet<IVec3>> = Vec::new();

    for (_, (a, b)) in distances.iter() {
        let posa = clusters.iter().position(|cluster| cluster.contains(a));
        let posb = clusters.iter().position(|cluster| cluster.contains(b));
        if let (Some(posa), Some(posb)) = (posa, posb) {
            if posa != posb {
                let tmp = clusters[posb].clone();
                for node in tmp {
                    clusters[posa].insert(node);
                }
                clusters.remove(posb);
            }
        } else if posa.is_none() && posb.is_none() {
            let cluster = HashSet::from([*a, *b]);
            clusters.push(cluster);
        } else if let Some(pos0) = posa {
            clusters[pos0].insert(*b);
        } else if let Some(pos1) = posb {
            clusters[pos1].insert(*a);
        }

        if clusters.len() == 1 && clusters[0].len() == nodes {
            return a.x * b.x;
        }
    }

    panic!("no solution");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let data = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
        assert_eq!(solve(data), 25272);
    }

    #[test]
    fn test2() {
        let data = std::fs::read_to_string("input/input.txt").unwrap();
        assert_eq!(solve(&data), 673096646);
    }
}
