use std::collections::{HashMap, HashSet};

fn parse_input(input: &Vec<String>) -> HashMap<String, Vec<String>> {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    input.iter().for_each(|line| {
        let seps: Vec<&str> = line.split("-").collect();

        if let Some(caves) = graph.get_mut(seps[0]) {
            caves.push(seps[1].to_string());
        } else {
            graph.insert(seps[0].to_string(), vec![seps[1].to_string()]);
        }

        if let Some(caves) = graph.get_mut(seps[1]) {
            caves.push(seps[0].to_string());
        } else {
            graph.insert(seps[1].to_string(), vec![seps[0].to_string()]);
        }
    });

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

    let mut visited = HashSet::new();

    let res = dfs(&graph, "start".to_string(), &mut visited);

    res == 3510
}

pub fn part2(_input: &Vec<String>) -> bool {
    unimplemented!();
}
