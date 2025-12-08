use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("input.txt").expect("fail to read input");

    println!("{:?}", part1(&parse(&input), 1000, 3));
    println!("{:?}", part2(&parse(&input)));
}

fn part2(points: &Vec<Point>) -> u64 {
    let mut segments = Vec::new();

    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let (p1, p2) = (&points[i], &points[j]);
            segments.push(Segment::new(p1, p2));
        }
    }

    segments.sort_by(|p1, p2| p1.d.cmp(&p2.d));

    let mut circuits: HashMap<&Point, i64> = HashMap::new();
    let mut new_id = 0;
    let mut last_conn: Option<&Segment> = None;

    for seg in segments.iter() {
        match (circuits.get(seg.p1).copied(), circuits.get(seg.p2).copied()) {
            (None, None) => {
                circuits.insert(seg.p1, new_id);
                circuits.insert(seg.p2, new_id);
                new_id += 1;
            }
            (None, Some(id)) => {
                circuits.insert(seg.p1, id);
            }
            (Some(id), None) => {
                circuits.insert(seg.p2, id);
            }
            (Some(id1), Some(id2)) if id1 != id2 => {
                for id in circuits.values_mut() {
                    if *id == id2 {
                        *id = id1;
                    }
                }
            }
            _ => {}
        }

        if points.len() == circuits.len() {
            last_conn = Some(seg);
            break;
        }
    }

    let last_conn = last_conn.unwrap();

    (last_conn.p1.x * last_conn.p2.x) as u64
}

fn part1(points: &Vec<Point>, connections: usize, count: usize) -> u64 {
    let mut segments = Vec::new();

    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let (p1, p2) = (&points[i], &points[j]);
            segments.push(Segment::new(p1, p2));
        }
    }

    segments.sort_by(|p1, p2| p1.d.cmp(&p2.d));

    let mut circuits: HashMap<&Point, i64> = HashMap::new();
    let mut new_id = 0;

    for seg in segments.iter().take(connections) {
        match (circuits.get(seg.p1).copied(), circuits.get(seg.p2).copied()) {
            (None, None) => {
                circuits.insert(seg.p1, new_id);
                circuits.insert(seg.p2, new_id);
                new_id += 1;
            }
            (None, Some(id)) => {
                circuits.insert(seg.p1, id);
            }
            (Some(id), None) => {
                circuits.insert(seg.p2, id);
            }
            (Some(id1), Some(id2)) if id1 != id2 => {
                for id in circuits.values_mut() {
                    if *id == id2 {
                        *id = id1;
                    }
                }
            }
            _ => {}
        }
    }

    let mut id_to_len: HashMap<i64, u64> = HashMap::new();

    for id in circuits.values() {
        *id_to_len.entry(*id).or_insert(0) += 1;
    }

    let mut lengths: Vec<u64> = id_to_len.values().copied().collect();
    lengths.sort();

    lengths.iter().rev().take(count).product()
}

fn distance(a: &Point, b: &Point) -> i64 {
    (a.x - b.x).pow(2) + (a.y - b.y).pow(2) + (a.z - b.z).pow(2)
}

#[derive(Debug, PartialEq, Clone)]
struct Segment<'a> {
    p1: &'a Point,
    p2: &'a Point,
    d: i64,
}

impl<'a> Segment<'a> {
    fn new(p1: &'a Point, p2: &'a Point) -> Self {
        Self {
            p1,
            p2,
            d: distance(p1, p2),
        }
    }
}

fn parse(input: &str) -> Vec<Point> {
    let mut res = Vec::new();

    for line in input.lines() {
        let parts: Vec<i64> = line
            .splitn(3, ',')
            .map(|s| s.parse().expect("invalid number"))
            .collect();
        res.push(Point::new(parts[0], parts[1], parts[2]));
    }

    res
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            parse("1,2,3\n4,5,6"),
            vec![Point::new(1, 2, 3), Point::new(4, 5, 6)]
        );
    }

    #[test]
    fn test_distance() {
        assert_eq!(distance(&Point::new(0, 0, 0), &Point::new(3, 4, 0)), 25);
    }

    const INPUT: &str = r#"162,817,812
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
425,690,689"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(INPUT), 10, 3), 40);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(INPUT)), 25272);
    }
}
