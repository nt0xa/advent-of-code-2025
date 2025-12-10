use std::fs;

use z3::{Solver, ast::Int};

fn main() {
    let input = fs::read_to_string("input.txt").expect("invalid input file");
    println!("part1: {:?}", part1(&parse1(&input)));
    println!("part2: {:?}", part2(&parse2(&input)));
}

fn part2(indicators: &Vec<IndicatorJoltage>) -> u64 {
    let mut total = 0;
    for indicator in indicators {
        let solver = Solver::new();

        let presses: Vec<_> = indicator
            .buttons
            .iter()
            .enumerate()
            .map(|(i, _)| Int::new_const(format!("b{}", i)))
            .collect();

        for p in &presses {
            solver.assert(p.ge(Int::from_u64(0)));
        }

        for (i, value) in indicator.joltages.iter().enumerate() {
            let mut items = Vec::new();
            for (j, button) in indicator.buttons.iter().enumerate() {
                if button.contains(&i) {
                    items.push(&presses[j]);
                }
            }
            solver.assert(
                items
                    .iter()
                    .fold(Int::from_u64(0), |sum, i| sum + *i)
                    .eq(Int::from_u64(*value as u64)),
            );
        }

        let mut min_presses = u64::MAX;

        for solution in solver.solutions(presses, false).take(1000) {
            let sum: u64 = solution.iter().map(Int::as_u64).map(Option::unwrap).sum();
            if sum < min_presses {
                min_presses = sum;
            }
        }
        total += min_presses;
    }
    total
}

fn parse2(input: &str) -> Vec<IndicatorJoltage> {
    let mut res = Vec::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let buttons = parts[1..parts.len() - 1].to_vec();
        let joltages = parts[parts.len() - 1];

        res.push(IndicatorJoltage::new(
            buttons
                .iter()
                .map(|s| {
                    s[1..s.len() - 1]
                        .split(',')
                        .map(|ns| ns.parse::<usize>().expect("invalid button"))
                        .collect()
                })
                .collect(),
            joltages[1..joltages.len() - 1]
                .split(',')
                .map(|s| s.parse::<usize>().expect("invalid joltage"))
                .collect(),
        ));
    }

    res
}

#[derive(Debug, PartialEq, Clone)]
struct IndicatorJoltage {
    buttons: Vec<Vec<usize>>,
    joltages: Vec<usize>,
}

impl IndicatorJoltage {
    fn new(buttons: Vec<Vec<usize>>, joltages: Vec<usize>) -> Self {
        Self { buttons, joltages }
    }
}

fn part1(indicators: &Vec<Indicator>) -> u32 {
    let mut total: u32 = 0;

    for indicator in indicators {
        let count = indicator.buttons.iter().count();
        let mut min_toggles = indicator.buttons.len();

        for n in 0..2_u32.pow(count as u32) {
            let bits: Vec<u8> = (0..count).map(|i| ((n >> i) & 1) as u8).collect();
            let bits_count = bits.iter().filter(|&&b| b == 1).count();
            let state = bits
                .iter()
                .enumerate()
                .map(|(i, &k)| k as u32 * indicator.buttons[i])
                .fold(0, |acc, x| acc ^ x);

            if state == indicator.desired && bits_count < min_toggles {
                min_toggles = bits_count;
            }
        }
        total += min_toggles as u32;
    }

    total
}

fn parse1(input: &str) -> Vec<Indicator> {
    let mut res = Vec::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let desired = parts[0];
        let buttons = parts[1..parts.len() - 1].to_vec();

        res.push(Indicator::new(
            desired[1..desired.len() - 1]
                .chars()
                .map(|c| if c == '.' { 0 } else { 1 })
                .enumerate()
                .map(|(i, b)| 2_u32.pow(i as u32) * b)
                .sum(),
            buttons
                .iter()
                .map(|s| {
                    s[1..s.len() - 1]
                        .split(',')
                        .map(|ns| ns.parse::<u32>().expect("invalid button"))
                        .map(|i| 2_u32.pow(i))
                        .sum()
                })
                .collect(),
        ));
    }

    res
}

#[derive(Debug, PartialEq, Clone)]
struct Indicator {
    desired: u32,
    buttons: Vec<u32>,
}

impl Indicator {
    fn new(desired: u32, buttons: Vec<u32>) -> Self {
        Self { desired, buttons }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse1() {
        assert_eq!(
            parse1("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}"),
            vec![Indicator::new(
                6,
                vec![8, 10, 4, 12, 5, 3].into_iter().collect(),
            )],
        );
    }

    const INPUT: &str = r#"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse1(INPUT)), 7);
    }

    #[test]
    fn test_parse2() {
        assert_eq!(
            parse2("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}"),
            vec![IndicatorJoltage::new(
                vec![
                    vec![3],
                    vec![1, 3],
                    vec![2],
                    vec![2, 3],
                    vec![0, 2],
                    vec![0, 1],
                ],
                vec![3, 5, 4, 7],
            )],
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse2(INPUT)), 33);
    }
}
