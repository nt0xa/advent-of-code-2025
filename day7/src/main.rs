use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    let input = fs::read_to_string("input.txt").expect("fail to open input file");
    let map = parse(&input);
    println!("part1: {:?}", part1(&map));
    println!("part2: {:?}", part2(&map));
}

fn part2(map: &Vec<Vec<char>>) -> u64 {
    let (first_line, rest) = map.split_first().expect("at least 2 lines");
    let mut beam: HashMap<usize, u64> = HashMap::new();
    let start = first_line
        .iter()
        .position(|&c| c == 'S')
        .expect("must have start on the first line");
    beam.insert(start, 1);
    let cols = map[0].len();

    for line in rest {
        let positions: Vec<(usize, u64)> = beam.iter().map(|(&k, &v)| (k, v)).collect();

        beam.clear();

        for (i, n) in positions {
            if line[i] == '^' {
                if i > 0 {
                    *beam.entry(i - 1).or_insert(0) += n;
                }

                if i < cols - 1 {
                    *beam.entry(i + 1).or_insert(0) += n;
                }
            } else {
                *beam.entry(i).or_insert(0) += n;
            }
        }
    }

    beam.values().sum()
}

fn part1(map: &Vec<Vec<char>>) -> u64 {
    let (first_line, rest) = map.split_first().expect("at least 2 lines");
    let mut beam: HashSet<usize> = HashSet::new();
    beam.insert(
        first_line
            .iter()
            .position(|&c| c == 'S')
            .expect("must have start on the first line"),
    );
    let cols = map[0].len();
    let mut splits = 0;

    for line in rest {
        let positions: Vec<usize> = beam.iter().copied().collect();

        for i in positions {
            if line[i] == '^' {
                splits += 1;
                beam.remove(&i);
                if i > 0 {
                    beam.insert(i - 1);
                }

                if i < cols - 1 {
                    beam.insert(i + 1);
                }
            }
        }
        // debug(line, &beam);
    }

    splits
}

fn debug(line: &Vec<char>, beam: &HashSet<usize>) {
    for (i, c) in line.iter().enumerate() {
        if beam.contains(&i) {
            print!("|");
        } else {
            print!("{}", c);
        }
    }
    println!();
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            parse("...\n.^."),
            vec![vec!['.', '.', '.'], vec!['.', '^', '.']],
        );
    }

    const INPUT: &str = r#".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(INPUT)), 21);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(INPUT)), 40);
    }
}
