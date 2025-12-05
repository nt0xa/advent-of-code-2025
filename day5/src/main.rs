use std::{cmp, collections::VecDeque, fs};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap_or_else(|_| panic!("fail to read input"));
    let (ranges, ids) = parse(&input);
    println!("part1: {:?}", part1(&ranges, &ids));
    println!("part2: {:?}", part2(&ranges, &ids));
}

fn part2(ranges: &Vec<(u64, u64)>, _: &Vec<u64>) -> u64 {
    let mut count = 0;
    let ranges = simplify_ranges(&ranges);

    for range in ranges {
        count += range.1 - range.0 + 1;
    }

    count
}

fn part1(ranges: &Vec<(u64, u64)>, ids: &Vec<u64>) -> u64 {
    let mut count = 0;
    let ranges = simplify_ranges(&ranges);

    for id in ids {
        let idx = ranges.binary_search(&(*id, *id)).unwrap_or_else(|i| i);

        if idx == 0 {
            if let Some(r) = ranges.first() {
                if contains(*r, *id) {
                    count += 1;
                }
            }
        } else if idx == ranges.len() {
            if let Some(r) = ranges.last() {
                if contains(*r, *id) {
                    count += 1;
                }
            }
        } else {
            if contains(*ranges.get(idx).unwrap(), *id)
                || contains(*ranges.get(idx - 1).unwrap(), *id)
            {
                count += 1;
            }
        }
    }

    count
}

fn simplify_ranges(ranges: &Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    let mut sorted = ranges.clone();
    sorted.sort();
    let mut queue: VecDeque<_> = sorted.into();
    let mut res = Vec::new();

    while let Some(r1) = queue.pop_front() {
        if let Some(r2) = queue.pop_front() {
            if let Some(r) = merge(r1, r2) {
                queue.push_front(r);
            } else {
                res.push(r1);
                queue.push_front(r2);
            }
        } else {
            res.push(r1);
        }
    }

    res
}

fn contains(r: (u64, u64), id: u64) -> bool {
    id >= r.0 && id <= r.1
}

fn merge(a: (u64, u64), b: (u64, u64)) -> Option<(u64, u64)> {
    if a.1 >= b.0 {
        Some((a.0, cmp::max(a.1, b.1)))
    } else {
        None
    }
}

fn parse(input: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    let mut ranges = Vec::new();
    let (ranges_str, ids_str) = input.split_once("\n\n").expect("invalid input");
    for line in ranges_str.lines() {
        let (min_str, max_str) = line
            .split_once('-')
            .unwrap_or_else(|| panic!("invalid range {}", line));
        ranges.push((
            min_str
                .parse::<u64>()
                .unwrap_or_else(|_| panic!("invalid number {}", min_str)),
            max_str
                .parse::<u64>()
                .unwrap_or_else(|_| panic!("invalid number {}", max_str)),
        ));
    }

    let mut ids = Vec::new();

    for line in ids_str.lines() {
        ids.push(
            line.parse::<u64>()
                .unwrap_or_else(|_| panic!("invalid id {}", line)),
        );
    }

    (ranges, ids)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"3-5
10-14
16-20
12-18

1
5
8
11
17
32"#;

    #[test]
    fn test_parse() {
        assert_eq!(
            parse(INPUT),
            (
                vec![(3, 5), (10, 14), (16, 20), (12, 18)],
                vec![1, 5, 8, 11, 17, 32]
            )
        );
    }

    #[test]
    fn test_merge() {
        assert_eq!(merge((1, 3), (3, 6)), Some((1, 6)));
        assert_eq!(merge((1, 2), (3, 6)), None);
        assert_eq!(merge((1, 4), (2, 6)), Some((1, 6)));
        assert_eq!(merge((1, 4), (1, 6)), Some((1, 6)));
        assert_eq!(merge((1, 2), (2, 4)), Some((1, 4)));
    }

    #[test]
    fn test_contains() {
        assert_eq!(contains((1, 4), 1), true);
        assert_eq!(contains((1, 4), 2), true);
        assert_eq!(contains((1, 4), 4), true);
        assert_eq!(contains((1, 4), 0), false);
        assert_eq!(contains((1, 4), 5), false);
    }

    #[test]
    fn test_simpify_ranges() {
        assert_eq!(
            simplify_ranges(&vec![(3, 5), (10, 14), (16, 20), (12, 18)]),
            vec![(3, 5), (10, 20)],
        );

        assert_eq!(
            simplify_ranges(&vec![(1, 10), (10, 14), (16, 20), (12, 18)]),
            vec![(1, 20)],
        );
    }

    #[test]
    fn test_part1() {
        let parsed = parse(INPUT);
        assert_eq!(part1(parsed.0, parsed.1), 3);
    }
}
