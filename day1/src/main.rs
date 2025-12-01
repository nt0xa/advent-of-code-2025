use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap_or_else(|_| panic!("failed to read input"));
    println!("part1: {:?}", part1(&input, 50));
    println!("part2: {:?}", part2(&input, 50));
}

fn part1(input: &str, initial: i32) -> i32 {
    let mut res = initial;
    let mut zeros = 0;
    let turns = parse(input);

    for n in turns {
        res = (res + n + 100) % 100;
        if res == 0 {
            zeros += 1;
        }
    }

    zeros
}

fn part2(input: &str, initial: i32) -> i32 {
    let mut res = initial;
    let mut zeros = 0;
    let turns = parse(input);

    for mut n in turns {
        zeros += n.abs() / 100;
        n = n % 100;
        if (res > 0 && res + n <= 0) || res + n >= 100 {
            zeros += 1;
        }
        res = (res + n).rem_euclid(100);
    }

    zeros
}

fn parse(input: &str) -> Vec<i32> {
    let mut res = Vec::new();
    let lines = input.lines();
    for line in lines {
        let mut n = 0;
        match line.chars().next().unwrap() {
            'L' => n = -1,
            'R' => n = 1,
            _ => {}
        }
        n = n * line[1..].parse::<i32>().unwrap();
        res.push(n);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT, 50), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT, 50), 6);
    }
}
