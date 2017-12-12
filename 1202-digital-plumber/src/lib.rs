extern crate petgraph;

use std::collections::HashSet;

use petgraph::Undirected;
use petgraph::graphmap::GraphMap;

pub fn groups(input: &[String]) -> u32 {
    let mut graph_map: GraphMap<u32, u32, Undirected> = GraphMap::new();
    for line in input {
        let mut parts = line.split(" <-> ");
        let node = parts.next().map(|n| n.parse::<u32>()).unwrap().unwrap();
        let connections: Vec<_> = parts.next().map(|line| {
            line.split(", ").filter_map(|n| n.parse::<u32>().ok()).collect()
        }).unwrap();

        graph_map.add_node(node);
        for conn in connections {
            graph_map.add_node(conn);
            graph_map.add_edge(node, conn, 1);
        }
    }

    let mut visited: HashSet<u32> = HashSet::new();
    let mut group_count = 0;
    for node in graph_map.nodes() {
        if visited.contains(&node) { continue; }
        let members = members_with(node, &graph_map);
        for member in members {
            visited.insert(member);
        }
        visited.insert(node);
        group_count += 1;
    }
    group_count
}

fn members_with(member: u32, graph_map: &GraphMap<u32, u32, Undirected>) -> Vec<u32> {
    let mut visited: HashSet<u32> = HashSet::new();
    let mut to_visit: Vec<_> = graph_map.neighbors(member).collect();
    while let Some(node) = to_visit.pop() {
        if visited.contains(&node) { continue; }
        graph_map.neighbors(node).for_each(|n| to_visit.push(n));
        visited.insert(node);
    }
    visited.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::groups;
    #[test]
    fn it_works() {
        let input = vec![
            String::from("0 <-> 2"),
            String::from("1 <-> 1"),
            String::from("2 <-> 0, 3, 4"),
            String::from("3 <-> 2, 4"),
            String::from("4 <-> 2, 3, 6"),
            String::from("5 <-> 6"),
            String::from("6 <-> 4, 5"),
        ];
        assert_eq!(groups(&input), 2);
    }
}
