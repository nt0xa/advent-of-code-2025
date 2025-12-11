use itertools::Itertools;
use std::{
    collections::{BTreeMap, HashMap, HashSet, VecDeque},
    fs,
};

fn main() {
    let input = fs::read_to_string("input.txt").expect("invalid input file");
    let graph = parse(&input);
    println!("part1: {:?}", part1(&graph));
    println!("part2: {:?}", part2(&graph));
}

fn part2(graph: &HashMap<&str, HashSet<&str>>) -> usize {
    let mut bottlenecks = find_bottlenecks(graph, 15);
    bottlenecks.insert("svr".to_owned());
    bottlenecks.insert("dac".to_owned());
    bottlenecks.insert("fft".to_owned());

    let mut bottlenecks: Vec<_> = bottlenecks
        .iter()
        .map(|node| (node, find_shortest_path(graph, "svr", node).unwrap().len()))
        .collect();

    bottlenecks.sort_by(|a, b| a.1.cmp(&b.1));
    let mut bridges: BTreeMap<usize, Vec<&str>> = BTreeMap::new();
    for (node, distance) in bottlenecks {
        bridges.entry(distance).or_insert(Vec::new()).push(node);
    }

    let items: Vec<_> = bridges.values().collect();

    let mut total: usize = 0;
    for path in items.iter().map(|v| v.iter()).multi_cartesian_product() {
        let mut current = 1;
        for (i, seg) in path.windows(2).enumerate() {
            let mut stops = vec![];
            if i + 1 <= items.len() - 1 {
                stops = items[i + 1].clone();
            }
            if i + 2 <= items.len() - 1 {
                stops.extend(items[i + 2]);
            }
            let count = find_all_paths(graph, seg[0], seg[1], &vec![], &stops).len();
            current *= count;
        }
        total += current;
    }

    total
}

fn part1(graph: &HashMap<&str, HashSet<&str>>) -> u64 {
    find_all_paths(graph, "you", "out", &vec![], &vec![]).len() as u64
}

fn find_bottlenecks(graph: &HashMap<&str, HashSet<&str>>, threshold: usize) -> HashSet<String> {
    let mut res = HashMap::new();

    for (node, edges) in graph {
        *res.entry(node).or_insert(0) += edges.len();
        for n in edges {
            *res.entry(n).or_insert(0) += 1;
        }
    }

    res.iter()
        .filter(|&(_, deg)| *deg >= threshold)
        .map(|(&node, _)| node)
        .map(|s| s.to_string())
        .collect()
}

fn find_shortest_path(
    graph: &HashMap<&str, HashSet<&str>>,
    start: &str,
    end: &str,
) -> Option<Vec<String>> {
    let mut queue: VecDeque<(&str, Vec<&str>)> = VecDeque::new();
    let mut visited: HashSet<&str> = HashSet::new();

    queue.push_back((start, vec![start]));
    visited.insert(start);

    while let Some((current, path)) = queue.pop_front() {
        if current == end {
            return Some(path.iter().map(|&s| s.to_string()).collect());
        }

        for neighbor in graph[current].iter() {
            if !visited.contains(neighbor) {
                visited.insert(neighbor);

                // Create new path with neighbor added
                let mut new_path = path.clone();
                new_path.push(neighbor);

                queue.push_back((neighbor, new_path));
            }
        }
    }
    None
}

fn find_all_paths(
    graph: &HashMap<&str, HashSet<&str>>,
    start: &str,
    end: &str,
    path: &Vec<&str>,
    stops: &[&str],
) -> Vec<HashSet<String>> {
    let mut path: Vec<&str> = path.clone();
    path.push(start);

    if start == end {
        return vec![path.iter().map(|s| s.to_string()).collect()];
    }

    if stops.contains(&start) {
        return vec![];
    }

    let mut all_paths = Vec::new();
    for neighbor in graph.get(start).unwrap() {
        if !path.contains(&neighbor) {
            let paths = find_all_paths(graph, neighbor, end, &path, stops);
            all_paths.extend(paths);
        }
    }
    all_paths
}

fn parse(input: &str) -> HashMap<&str, HashSet<&str>> {
    let mut graph = HashMap::new();
    for line in input.lines() {
        let (node, outputs) = line.split_once(": ").unwrap();
        graph.insert(node, HashSet::new());
        for out in outputs.split_whitespace() {
            graph.entry(node).and_modify(|entry| {
                entry.insert(out);
            });
            graph.entry(out).or_insert(HashSet::new());
        }
    }
    graph
}

fn graphviz(graph: &HashMap<&str, HashSet<&str>>, highlight: &[(&[&str], &str)]) -> String {
    let mut res = String::new();

    res.push_str("digraph Components {\n");

    for n in graph.keys() {
        let mut style = String::new();
        for (part, color) in highlight {
            if part.contains(&n) {
                style.push_str(&format!("[color={}]", color));
                break;
            }
        }

        res.push_str(&format!("\"{}\" {};\n", n, style));
    }

    for (from, edges) in graph.iter() {
        for to in edges {
            res.push_str(&format!("\"{}\" -> \"{}\";\n", from, to));
        }
    }

    res.push_str("}\n");

    return res;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            parse(&"a: b c"),
            HashMap::from([
                ("a", HashSet::from(["b", "c"])),
                ("b", HashSet::new()),
                ("c", HashSet::new())
            ]),
        );
    }

    const INPUT1: &str = r#"aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(INPUT1)), 5);
    }

    const INPUT2: &str = r#"svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out"#;

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(INPUT2)), 2);
    }
}
