use std::collections::{HashMap, HashSet, VecDeque};

use crate::util::lines_from_file;

pub fn day12() {
    println!("== Day 12 ==");
    let input = lines_from_file("src/day12/input.txt");
    let a = part_a(&input);
    println!("Part A: {}", a);
    let b = part_b(&input);
    println!("Part B: {}", b);
}

fn part_a(input: &Vec<String>) -> usize {
    let graph = to_graph(input);
    let paths = all_paths(&graph, "start", "end", false);
    paths.len()
}

fn part_b(input: &Vec<String>) -> usize {
    let graph = to_graph(input);
    let paths = all_paths(&graph, "start", "end", true);
    paths.len()
}

fn to_graph(input: &Vec<String>) -> HashMap<String, Vec<String>> {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    let i: Vec<Vec<&str>> = input.iter().map(|s| s.split("-").collect::<Vec<&str>>()).collect();
    for x in i {
        let key = x.get(0).unwrap();
        let value = x.get(1).unwrap();

        if !graph.contains_key(*key) {
            graph.insert(key.to_string(), Vec::new());
        }
        if !graph.contains_key(*value) {
            graph.insert(value.to_string(), Vec::new());
        }

        graph.get_mut(*key).unwrap_or(&mut Vec::new()).push(value.to_string());
        graph.get_mut(*value).unwrap_or(&mut Vec::new()).push(key.to_string());
    }
    graph
}


fn all_paths(graph: &HashMap<String, Vec<String>>, start: &str, end: &str, revisit_small: bool) -> Vec<Vec<String>> {
    let mut paths: Vec<Vec<String>> = Vec::new();
    let start_path: Vec<String> = vec![start.to_string()];

    let mut q: VecDeque<(&str, HashSet<&str>, Option<&str>, Vec<String>)> = VecDeque::new();
    let beginning = (start, HashSet::from([start]), None, start_path);
    q.push_back(beginning);

    while !q.is_empty() {
        let (current, small, revisit, path) = q.pop_front().unwrap();
        if current == end {
            paths.push(path.clone());
            continue;
        }
        let x = graph.get(current).unwrap();
        for n in x.iter() {
            if !small.contains(n.as_str()) {
                let mut new_visited: HashSet<&str> = small.clone();
                if n.to_lowercase() == n.to_string() {
                    new_visited.insert(n);
                }
                let mut new_path = path.clone();
                new_path.push(n.to_string());
                q.push_back((n, new_visited, revisit, new_path));
            } else if small.contains(n.as_str()) && revisit.is_none() && ![start, end].contains(&n.as_str()) && revisit_small {
                let mut new_path = path.clone();
                new_path.push(n.to_string());
                q.push_back((n, small.clone(), Some(n), new_path));
            }
        }
    }

    paths
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_graph_t() {
        let input: Vec<String> = vec![
            "start-A",
            "start-b",
            "A-c",
            "A-b",
            "b-d",
            "A-end",
            "b-end",
        ].iter()
            .map(|s| s.to_string())
            .collect();

        let expected: HashMap<String, Vec<String>> = HashMap::from([
            ("start".to_string(), vec!["A".to_string(), "b".to_string()]),
            ("A".to_string(), vec!["start".to_string(), "c".to_string(), "b".to_string(), "end".to_string()]),
            ("b".to_string(), vec!["start".to_string(), "A".to_string(), "d".to_string(), "end".to_string()]),
            ("c".to_string(), vec!["A".to_string()]),
            ("d".to_string(), vec!["b".to_string()]),
            ("end".to_string(), vec!["A".to_string(), "b".to_string()])
        ]);

        assert_eq!(expected, to_graph(&input))
    }

    #[test]
    fn all_paths_t() {
        let input: Vec<String> = vec![
            "start-A",
            "start-b",
            "A-c",
            "A-b",
            "b-d",
            "A-end",
            "b-end",
        ].iter()
            .map(|s| s.to_string())
            .collect();

        let graph = to_graph(&input);
        let mut paths = all_paths(&graph, "start", "end", false);
        let mut expected: Vec<Vec<String>> = vec![
            vec!["start", "A", "b", "A", "c", "A", "end"],
            vec!["start", "A", "b", "A", "end"],
            vec!["start", "A", "b", "end"],
            vec!["start", "A", "c", "A", "b", "A", "end"],
            vec!["start", "A", "c", "A", "b", "end"],
            vec!["start", "A", "c", "A", "end"],
            vec!["start", "A", "end"],
            vec!["start", "b", "A", "c", "A", "end"],
            vec!["start", "b", "A", "end"],
            vec!["start", "b", "end"],
        ].iter().map(|v| v.iter().map(|s| s.to_string()).collect()).collect();

        expected.sort();
        paths.sort();

        assert_eq!(10, paths.len());
        assert_eq!(expected, paths);

        let paths_b = all_paths(&graph, "start", "end", true);
        assert_eq!(36, paths_b.len());
    }

    #[test]
    fn some_tests() {
        let str = "A";
        let string = "A".to_string();
        assert_eq!(str, string);
        assert_eq!(true, str == string);
        assert_ne!("A", "a");
        assert_eq!("a".to_lowercase(), "a");
        assert_ne!("A".to_lowercase(), "A");

        let set: HashSet<&str> = HashSet::from(["a"]);
        assert_eq!(true, set.contains("a"));
        assert_eq!(false, set.contains("A"));
    }

    #[test]
    fn part_a_small_input() {
        let input: Vec<String> = vec![
            "start-A",
            "start-b",
            "A-c",
            "A-b",
            "b-d",
            "A-end",
            "b-end",
        ].iter()
            .map(|s| s.to_string())
            .collect();

        let result = part_a(&input);
        assert_eq!(10, result)
    }

    #[test]
    fn part_a_test_input() {
        let filename = "src/day12/test-input.txt";
        let input = lines_from_file(filename);
        let result = part_a(&input);
        assert_eq!(226, result);
    }

    #[test]
    fn part_a_real() {
        let filename = "src/day12/input.txt";
        let input = lines_from_file(filename);
        let result = part_a(&input);
        assert_eq!(4720, result);
    }

    #[test]
    fn part_b_real() {
        let filename = "src/day12/input.txt";
        let input = lines_from_file(filename);
        let result = part_b(&input);
        assert_eq!(147848, result);
    }
}