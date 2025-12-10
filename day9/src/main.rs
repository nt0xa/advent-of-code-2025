use std::{
    cmp,
    collections::HashSet,
    fs::{self},
};

fn main() {
    let input = fs::read_to_string("input.txt").expect("invalid input file");
    let parsed = parse(&input);
    println!("part1: {:?}", part1(&parsed));
    println!("part2: {:?}", part2(&parsed));
}

fn part2(points: &Vec<(u64, u64)>) -> u64 {
    let mut vertical = Vec::new();
    let mut vertical_set = HashSet::new();
    let mut horizontal = Vec::new();
    let mut horizontal_set = HashSet::new();
    let rows = *points.iter().map(|(x, _)| x).max().unwrap();
    let cols = *points.iter().map(|(_, y)| y).max().unwrap();
    let mut max_area = 0;

    for i in 0..points.len() {
        let p1 = points[i];
        let p2 = points[(i + 1) % points.len()];

        if p1.0 == p2.0 {
            let v = ((cmp::min(p1.1, p2.1), cmp::max(p1.1, p2.1)), p1.0);
            vertical.push(v);
            vertical_set.insert(v);
        } else if p1.1 == p2.1 {
            let v = ((cmp::min(p1.0, p2.0), cmp::max(p1.0, p2.0)), p1.1);
            horizontal.push(v);
            horizontal_set.insert(v);
        } else {
            panic!("invalid points");
        }
    }
    vertical.sort();
    horizontal.sort();

    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let p1 = points[i];
            let p2 = points[j];

            let cur_area = area(p1, p2);
            if cur_area < max_area {
                continue;
            }

            if !is_inside(
                &points,
                &vertical,
                &vertical_set,
                &horizontal,
                &horizontal_set,
                rows,
                cols,
                p1,
                p2,
            ) {
                continue;
            }

            max_area = cur_area;
        }
    }

    max_area
}

fn is_inside(
    p: &Vec<(u64, u64)>,
    v: &Vec<((u64, u64), u64)>,
    vs: &HashSet<((u64, u64), u64)>,
    h: &Vec<((u64, u64), u64)>,
    hs: &HashSet<((u64, u64), u64)>,
    rows: u64,
    cols: u64,
    p1: (u64, u64),
    p2: (u64, u64),
) -> bool {
    let col_min = cmp::min(p1.0, p2.0);
    let col_max = cmp::max(p1.0, p2.0);
    let row_min = cmp::min(p1.1, p2.1);
    let row_max = cmp::max(p1.1, p2.1);

    is_edge_inside(v, hs, col_min, col_max, row_min, rows)
        && is_edge_inside(v, hs, col_min, col_max, row_max, rows)
        && is_edge_inside(h, vs, row_min, row_max, col_min, cols)
        && is_edge_inside(h, vs, row_min, row_max, col_max, cols)
}

fn is_edge_inside(
    perpendicular: &Vec<((u64, u64), u64)>,
    parallel: &HashSet<((u64, u64), u64)>,
    from: u64,
    to: u64,
    value: u64,
    max_value: u64,
) -> bool {
    let pos = perpendicular
        .binary_search(&((value, max_value + 1), from))
        .unwrap_or_else(|x| x);

    let mut possible = perpendicular[0..pos].to_vec();
    possible.sort_by(|((_, a), _), ((_, b), _)| a.cmp(&b));

    let pos = possible
        .binary_search_by(|((b, a), _)| a.cmp(&value).then(b.cmp(&0)))
        .unwrap_or_else(|x| x);

    let mut intersected: Vec<u64> = possible[pos..].iter().map(|(_, v)| *v).collect();

    intersected.sort();

    if intersected.is_empty() {
        return false;
    }

    let intersected = match intersected.binary_search(&from) {
        Ok(pos) => intersected[pos..].to_vec(),
        Err(pos) if pos > 0 => intersected[pos - 1..].to_vec(),
        Err(0) => return false,
        _ => panic!("unexpected"),
    };

    let intersected = match intersected.binary_search(&to) {
        Ok(pos) => intersected[..=pos].to_vec(),
        Err(pos) if pos < intersected.len() => intersected[..=pos].to_vec(),
        Err(pos) if pos == intersected.len() => return false,
        _ => panic!("unexpected"),
    };

    for i in 0..intersected.len() - 1 {
        let start = intersected[i];
        let end = intersected[i + 1];

        if parallel.contains(&((start, end), value)) {
            continue;
        }

        let len = end - start;
        if len == 1 {
            return false;
        }

        let count = intersected.len() - i + 1
            - intersected[i + 1..]
                .windows(2)
                .map(|w| ((w[0], w[1]), value))
                .filter(|seg| parallel.contains(seg))
                .count();

        if count % 2 != 1 {
            return false;
        }
    }

    true
}

fn part1(points: &Vec<(u64, u64)>) -> u64 {
    let mut max_square = 0;
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let sq = area(points[i], points[j]);
            if sq > max_square {
                max_square = sq;
            }
        }
    }

    max_square
}

fn area(p1: (u64, u64), p2: (u64, u64)) -> u64 {
    (p1.0.abs_diff(p2.0) + 1) * (p1.1.abs_diff(p2.1) + 1)
}

fn parse(input: &str) -> Vec<(u64, u64)> {
    input
        .lines()
        .map(|line| {
            line.split_once(',')
                .map(|(s1, s2)| {
                    (
                        s1.parse::<u64>().expect("invalid number"),
                        s2.parse::<u64>().expect("invalid number"),
                    )
                })
                .expect("invalid line")
        })
        .collect()
}

fn to_svg(
    points: &Vec<(u64, u64)>,
    rect: Option<((u64, u64), (u64, u64))>,
    v: Option<&Vec<((u64, u64), u64)>>,
    h: Option<&Vec<((u64, u64), u64)>>,
) -> String {
    let min_x = points.iter().map(|(x, _)| *x).min().unwrap();
    let max_x = points.iter().map(|(x, _)| *x).max().unwrap();
    let min_y = points.iter().map(|(_, y)| *y).min().unwrap();
    let max_y = points.iter().map(|(_, y)| *y).max().unwrap();

    let padding = 10;
    let viewbox_x = min_x.saturating_sub(padding);
    let viewbox_y = min_y.saturating_sub(padding);
    let viewbox_width = max_x - min_x + 2 * padding;
    let viewbox_height = max_y - min_y + 2 * padding;

    let mut path = String::new();
    let mut extra = String::new();
    for (i, (x, y)) in points.iter().enumerate() {
        if i == 0 {
            path.push_str(&format!("M {},{}", x, y));
        } else {
            path.push_str(&format!(" L {},{}", x, y));
        }
    }
    path.push_str(" Z");

    if let Some(v) = v {
        for ((y1, y2), x) in v {
            extra.push_str(&format!(
                r#"
  <line x1="{}" y1="{}" x2="{}" y2="{}" stroke="red" stroke-width="50" />"#,
                x, y1, x, y2,
            ));
        }
    }

    if let Some(h) = h {
        for ((x1, x2), y) in h {
            extra.push_str(&format!(
                r#"
  <line x1="{}" y1="{}" x2="{}" y2="{}" stroke="red" stroke-width="50" />"#,
                x1, y, x2, y,
            ));
        }
    }

    if let Some(((x1, y1), (x2, y2))) = rect {
        extra.push_str(&format!(
            r#"
  <rect x="{}" y="{}" width="{}" height="{}" fill="none" stroke="blue" stroke-width="30" />"#,
            x1.min(x2),
            y1.min(y2),
            x1.abs_diff(x2) + 1,
            y1.abs_diff(y2) + 1,
        ));
    }

    format!(
        r#"<svg viewBox="{} {} {} {}" xmlns="http://www.w3.org/2000/svg">
  <path d="{}" fill="cyan" stroke="none" />
  {}
</svg>"#,
        viewbox_x, viewbox_y, viewbox_width, viewbox_height, path, extra
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(parse("1,2\n3,4"), vec![(1, 2), (3, 4)]);
    }

    const INPUT: &str = r#"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(INPUT)), 50);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(INPUT)), 24);
    }
}
