use core::fmt;
use std::fs;

fn main() {
    let input =
        fs::read_to_string("input.txt").unwrap_or_else(|_| panic!("failed to read input file"));
    let map = parse(&input);
    println!("part1: {:?}", part1(&map));
    println!("part2: {:?}", part2(&mut map.clone()));
}

fn part2(map: &mut Map) -> u64 {
    let mut removed = 0;
    let mut to_remove = Vec::new();

    loop {
        for row in 0..map.rows() {
            for col in 0..map.cols() {
                if Some('@') != map.get(row, col) {
                    continue;
                }

                let adjacent_count = map.count_adjacent_paper(row, col);

                if adjacent_count < 4 {
                    to_remove.push((row, col))
                }
            }
        }

        if to_remove.is_empty() {
            break;
        }

        while let Some((row, col)) = to_remove.pop() {
            map.set(row, col, 'x');
            removed += 1;
        }
    }
    removed
}

fn part1(map: &Map) -> u64 {
    let mut accessible = 0;

    for row in 0..map.rows() {
        for col in 0..map.cols() {
            if Some('@') != map.get(row, col) {
                continue;
            }

            let adjacent_count = map.count_adjacent_paper(row, col);
            if adjacent_count < 4 {
                accessible += 1;
            }
        }
    }


    accessible
}

fn parse(input: &str) -> Map {
    Map::new(input.lines().map(|line| line.chars().collect()).collect())
}

#[derive(PartialEq, Clone)]
struct Map(Vec<Vec<char>>);

impl Map {
    fn new(data: Vec<Vec<char>>) -> Self {
        Self(data)
    }

    fn rows(&self) -> usize {
        self.0.len()
    }

    fn cols(&self) -> usize {
        self.0.first().map_or(0, |row| row.len())
    }

    fn set(&mut self, row: usize, col: usize, value: char) {
        self.0
            .get_mut(row)
            .and_then(|row| row.get_mut(col))
            .map(|v| *v = value);
    }

    fn get(&self, row: usize, col: usize) -> Option<char> {
        self.0.get(row).and_then(|row| row.get(col)).copied()
    }

    fn count_if_paper(&self, row: usize, col: usize) -> u64 {
        match self.get(row, col) {
            Some('@') => 1,
            _ => 0,
        }
    }

    fn count_adjacent_paper(&self, row: usize, col: usize) -> u64 {
        let mut res = 0;
        let pos = Position::from_indices(row, col);

        for d in Direction::ALL {
            if let Some((row, col)) = pos.move_to(d).to_indices() {
                res += self.count_if_paper(row, col)
            }
        }

        res
    }
}

impl fmt::Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.rows() {
            for col in 0..self.cols() {
                write!(f, "{}", self.get(row, col).unwrap())?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
struct Position {
    row: i32,
    col: i32,
}

impl Position {
    fn move_to(&self, direction: Direction) -> Position {
        let delta = direction.delta();
        Position {
            row: self.row + delta.row,
            col: self.col + delta.col,
        }
    }

    fn from_indices(row: usize, col: usize) -> Self {
        Self {
            row: row.try_into().expect("row index too large for i32"),
            col: col.try_into().expect("col index too large for i32"),
        }
    }

    fn to_indices(&self) -> Option<(usize, usize)> {
        if self.row >= 0 && self.col >= 0 {
            Some((self.row as usize, self.col as usize))
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq)]
struct Delta {
    row: i32,
    col: i32,
}

impl Delta {
    fn new(row: i32, col: i32) -> Self {
        Self { row, col }
    }
}

#[derive(Debug, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
    NorthWest,
    NorthEast,
    SouthWest,
    SouthEast,
}

impl Direction {
    const ALL: [Self; 8] = [
        Direction::North,
        Direction::NorthEast,
        Direction::NorthWest,
        Direction::South,
        Direction::SouthEast,
        Direction::SouthWest,
        Direction::East,
        Direction::West,
    ];

    fn delta(&self) -> Delta {
        match self {
            Direction::North => Delta::new(-1, 0),
            Direction::South => Delta::new(1, 0),
            Direction::East => Delta::new(0, 1),
            Direction::West => Delta::new(0, -1),
            Direction::NorthWest => Delta::new(-1, -1),
            Direction::NorthEast => Delta::new(-1, 1),
            Direction::SouthWest => Delta::new(1, -1),
            Direction::SouthEast => Delta::new(1, 1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            parse("..@\n.@."),
            Map::new(vec![vec!['.', '.', '@'], vec!['.', '@', '.']])
        );
    }

    const INPUT: &str = r#"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(INPUT)), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part1(&parse(INPUT)), 43);
    }
}
