use std::collections::{HashMap, HashSet};

fn parse_input(input: &Vec<String>) -> HashMap<String, Vec<String>> {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    input.iter().for_each(|line| {
        let seps: Vec<&str> = line.split("-").collect();

        if seps[1] != "start" {
            graph
                .entry(seps[0].to_string())
                .or_insert(vec![])
                .push(seps[1].to_string());
        }

        if seps[0] != "start" {
            graph
                .entry(seps[1].to_string())
                .or_insert(vec![])
                .push(seps[0].to_string());
        }
    });

    graph.remove("end");

    graph
}

fn dfs(graph: &HashMap<String, Vec<String>>, cave: String, visited: &mut HashSet<String>) -> usize {
    if cave == "end" {
        return 1;
    }
    let is_small = cave.to_lowercase() == cave;
    if is_small {
        visited.insert(cave.clone());
    }

    let mut res = 0;
    if let Some(caves) = graph.get(&cave) {
        for c in caves {
            if !visited.contains(c) {
                res += dfs(graph, c.to_string(), visited);
            }
        }
    }

    if is_small {
        visited.remove(&cave);
    }

    res
}

pub fn part1(input: &Vec<String>) -> bool {
    let graph = parse_input(input);

    let res = dfs(&graph, "start".to_string(), &mut HashSet::new());

    res == 3510
}

fn dfs2(
    graph: &HashMap<String, Vec<String>>,
    cave: String,
    visited: &mut HashSet<String>,
    twice: Option<&String>,
) -> usize {
    if cave == "end" {
        return 1;
    }
    let is_small = cave.to_lowercase() == cave;
    if is_small {
        visited.insert(cave.clone());
    }

    let mut res = 0;
    if let Some(caves) = graph.get(&cave) {
        for c in caves {
            if !visited.contains(c) {
                res += dfs2(graph, c.to_string(), visited, twice);
            } else if twice.is_none() {
                res += dfs2(graph, c.to_string(), visited, Some(c));
            }
        }
    }

    if is_small && twice.map_or(true, |c| c != &cave) {
        visited.remove(&cave);
    }

    res
}

pub fn part2(input: &Vec<String>) -> bool {
    let graph = parse_input(input);

    let res = dfs2(&graph, "start".to_string(), &mut HashSet::new(), None);

    res == 122880
}
