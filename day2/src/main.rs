use std::{cmp, collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap_or_else(|_| panic!("fail to read input"));
    println!("part1: {:?}", part1(&input));
    println!("part2: {:?}", part2(&input));
}

fn part2(input: &str) -> u64 {
    let mut sum = 0;
    let ranges = parse(&input);
    for range in ranges {
        let mut set = HashSet::new();
        let digits_left = digits_num(range.0);
        let digits_right = digits_num(range.1);
        for digits_count in digits_left..=digits_right {
            let divs = divisors(digits_count);
            for divisor in divs {
                // We need at least 2 repetitions.
                if divisor == digits_count {
                    continue;
                }
                let repeat_count = digits_count / divisor;

                let left = cmp::max(
                    10_u64.pow(divisor - 1),
                    range.0 / 10_u64.pow(digits_count - divisor),
                );
                let right = cmp::min(
                    10_u64.pow(divisor) - 1,
                    range.1 / 10_u64.pow(digits_count - divisor),
                );

                for d in left..=right {
                    let n = repeat_num(d, divisor, repeat_count);
                    if n >= range.0 && n <= range.1 {
                        set.insert(n);
                    }
                }
            }
        }

        for n in set {
            sum += n;
        }
    }
    sum
}

fn part1(input: &str) -> u64 {
    let mut sum = 0;
    let ranges = parse(&input);

    for range in ranges {
        let digits_left = digits_num(range.0);
        let digits_right = digits_num(range.1);

        for i in digits_left..=digits_right {
            // Skip numbers with odd number of digits.
            if i % 2 != 0 {
                continue;
            }
            let d = i / 2;

            let left = cmp::max(range.0, 10_u64.pow(i - 1)) / 10_u64.pow(d);
            let right = cmp::min(range.1, repeat_num(9, 1, i)) / 10_u64.pow(d);

            for half in left..=right {
                let n = repeat_num(half, d, 2);
                if n >= range.0 && n <= range.1 {
                    sum += n;
                }
            }
        }
    }

    sum
}

fn divisors(num: u32) -> Vec<u32> {
    let mut divs = Vec::new();
    let mut i = 1;
    while i * i <= num {
        if num % i == 0 {
            divs.push(i);
            if i != num / i {
                divs.push(num / i);
            }
        }
        i += 1;
    }
    divs.sort();
    divs
}

fn repeat_num(num: u64, digits: u32, count: u32) -> u64 {
    let mut res = num;
    for _ in 0..count - 1 {
        res = res * 10_u64.pow(digits) + num;
    }
    res
}

fn digits_num(num: u64) -> u32 {
    let mut n = num;
    let mut res = 0;

    while n > 0 {
        n /= 10;
        res += 1;
    }

    res
}

fn parse(input: &str) -> Vec<(u64, u64)> {
    let mut ranges = Vec::new();

    for range_str in input.trim_end().split(',') {
        let (left_str, right_str) = range_str
            .split_once('-')
            .unwrap_or_else(|| panic!("invalid range {}", range_str));

        let left = left_str
            .parse::<u64>()
            .unwrap_or_else(|_| panic!("invalid left {:?}", left_str));

        let right = right_str
            .parse::<u64>()
            .unwrap_or_else(|_| panic!("invalid right {}", right_str));

        ranges.push((left, right));
    }

    ranges
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digits_num() {
        assert_eq!(digits_num(1), 1);
        assert_eq!(digits_num(10), 2);
        assert_eq!(digits_num(99), 2);
        assert_eq!(digits_num(123123), 6);
    }

    #[test]
    fn test_repeat_num() {
        assert_eq!(repeat_num(9, 1, 1), 9);
        assert_eq!(repeat_num(9, 1, 2), 99);
        assert_eq!(repeat_num(9, 1, 10), 9999999999);
        assert_eq!(repeat_num(123, 3, 2), 123123);
        assert_eq!(repeat_num(12, 2, 3), 121212);
    }

    #[test]
    fn test_divisors() {
        assert_eq!(divisors(10), vec![1, 2, 5, 10]);
        assert_eq!(divisors(11), vec![1, 11]);
        assert_eq!(divisors(36), vec![1, 2, 3, 4, 6, 9, 12, 18, 36]);
    }

    const INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,\
1698522-1698528,446443-446449,38593856-38593862,565653-565659,\
824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 1227775554)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 4174379265)
    }
}
