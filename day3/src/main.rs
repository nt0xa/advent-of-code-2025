use std::fs;

fn main() {
    let input =
        fs::read_to_string("input.txt").unwrap_or_else(|err| panic!("invalid input file: {}", err));
    let parsed = parse(&input);
    println!("part1: {:?}", part1(&parsed));
    println!("part2: {:?}", part2(&parsed));
}

const COUNT: usize = 12;

fn part2(banks: &Vec<Vec<u32>>) -> u64 {
    let mut sum = 0;
    for bank in banks {
        let mut max_vs: [u32; COUNT] = [0; COUNT];
        let mut last_idx: i64 = -1;

        for vi in 0..COUNT {
            for i in (last_idx + 1) as usize..bank.len() - (COUNT - vi) + 1 {
                let vmax = max_vs.get(vi).unwrap();
                let v = bank.get(i).unwrap();
                if v > vmax {
                    max_vs[vi] = *v;
                    last_idx = i as i64;
                }

                if v == &9 {
                    break;
                }
            }
        }
        let mut bank_v = 0;
        for (i, v) in max_vs.iter().rev().enumerate() {
            bank_v += 10_u64.pow(i as u32) * (*v as u64);
        }
        sum += bank_v;
    }
    sum
}

fn part1(banks: &Vec<Vec<u32>>) -> u64 {
    let mut sum = 0;

    for bank in banks {
        let mut max_left = 0;
        let mut max_left_idx = 0;
        for i in 0..bank.len() - 1 {
            let v = bank.get(i).unwrap();
            if v > &max_left {
                max_left = *v;
                max_left_idx = i;
            }
            if max_left == 9 {
                break;
            }
        }

        let mut max_right = 0;
        for i in max_left_idx + 1..bank.len() {
            let v = bank.get(i).unwrap();
            if v > &max_right {
                max_right = *v;
            }
            if max_right == 9 {
                break;
            }
        }

        sum += (max_left * 10 + max_right) as u64;
    }

    sum
}

fn parse(input: &str) -> Vec<Vec<u32>> {
    let mut res = Vec::new();

    for line in input.lines() {
        let mut nums = Vec::new();
        for char in line.chars() {
            nums.push(
                char.to_digit(10)
                    .unwrap_or_else(|| panic!("invalid number {}", char)),
            );
        }
        res.push(nums);
    }

    res
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &str = r#"987654321111111
811111111111119
234234234234278
818181911112111"#;

    #[test]
    fn test_parse() {
        assert_eq!(parse("123\n321"), vec![vec![1, 2, 3], vec![3, 2, 1]])
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(INPUT)), 357);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(INPUT)), 3121910778619);
    }
}
