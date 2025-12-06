use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt")
        .unwrap_or_else(|err| panic!("fail to read input file: {}", err));
    println!("part1: {:?}", calc(&parse1(&input)));
    println!("part2: {:?}", calc(&parse2(&input)));
}

fn parse2(input: &str) -> Vec<Column> {
    let mut res = Vec::new();
    let lines: Vec<&str> = input.lines().collect();
    let (op_line, num_lines) = lines.split_last().unwrap();
    let cols = op_line.split_whitespace().count();
    let mut nums: Vec<Vec<u64>> = vec![Vec::new(); cols];
    let line_len = lines.first().unwrap().len();

    let mut idx = 0;

    for i in (0..line_len).rev() {
        let num_str = num_lines
            .iter()
            .map(|line| line.chars().nth(i).unwrap())
            .collect::<String>()
            .trim()
            .to_string();

        if num_str.is_empty() {
            idx += 1;
            continue;
        }

        let num = num_str
            .trim()
            .parse::<u64>()
            .unwrap_or_else(|_| panic!("invalid number: {}", num_str));

        nums.get_mut(idx).unwrap().push(num);
    }

    for (i, op_str) in op_line.split_whitespace().rev().enumerate() {
        let op = match op_str {
            "*" => Operation::Mul,
            "+" => Operation::Add,
            _ => panic!("invalid operation: {}", op_str),
        };

        res.push(Column::new(nums.get(i).unwrap().clone(), op));
    }

    res
}

fn calc(cols: &Vec<Column>) -> u64 {
    let mut total = 0;

    for col in cols {
        total += match col.op {
            Operation::Mul => col.nums.iter().product::<u64>(),
            Operation::Add => col.nums.iter().sum(),
        };
    }

    total
}

fn parse1(input: &str) -> Vec<Column> {
    let mut lines = input.lines().peekable();
    let cols = lines
        .peek()
        .and_then(|line| Some(line.split_whitespace().count()))
        .expect("first line must contain some numbers");

    let mut nums: Vec<Vec<u64>> = vec![Vec::new(); cols];
    let mut res = Vec::new();

    while let Some(line) = lines.next() {
        if lines.peek().is_some() {
            for (i, num_str) in line.split_whitespace().enumerate() {
                nums.get_mut(i).unwrap().push(
                    num_str
                        .parse::<u64>()
                        .unwrap_or_else(|_| panic!("invalid number {}", num_str)),
                );
            }
        } else {
            // Last line
            for (i, op_str) in line.split_whitespace().enumerate() {
                let op = match op_str {
                    "*" => Operation::Mul,
                    "+" => Operation::Add,
                    _ => panic!("invalid operation: {}", op_str),
                };

                res.push(Column::new(nums.get(i).unwrap().clone(), op));
            }
        }
    }

    res
}

#[derive(Debug, PartialEq)]
struct Column {
    nums: Vec<u64>,
    op: Operation,
}

impl Column {
    fn new(nums: Vec<u64>, op: Operation) -> Self {
        Self { nums, op }
    }
}

#[derive(Debug, PartialEq)]
enum Operation {
    Mul,
    Add,
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  "#;

    #[test]
    fn test_parse1() {
        assert_eq!(
            parse1(INPUT),
            vec![
                Column::new(vec![123, 45, 6], Operation::Mul),
                Column::new(vec![328, 64, 98], Operation::Add),
                Column::new(vec![51, 387, 215], Operation::Mul),
                Column::new(vec![64, 23, 314], Operation::Add),
            ]
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(calc(&parse1(INPUT)), 4277556);
    }

    #[test]
    fn test_parse2() {
        assert_eq!(
            parse2(INPUT),
            vec![
                Column::new(vec![4, 431, 623], Operation::Add),
                Column::new(vec![175, 581, 32], Operation::Mul),
                Column::new(vec![8, 248, 369], Operation::Add),
                Column::new(vec![356, 24, 1], Operation::Mul),
            ]
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(calc(&parse2(INPUT)), 3263827);
    }
}
