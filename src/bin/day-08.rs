use aoc_2023::commons::io::Input;
use aoc_2023::commons::math::lcm;
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::EdgeRef;
use std::collections::{HashMap, HashSet};
use std::error::Error;

#[inline]
fn path_length<F: Fn(NodeIndex) -> bool>(
    graph: &DiGraph<&str, char>,
    path: &str,
    start_node: NodeIndex,
    end: F,
) -> usize {
    let mut current_node = start_node;
    for (i, dir) in path.chars().cycle().enumerate() {
        for edge_ref in graph.edges(current_node) {
            if *edge_ref.weight() == dir {
                current_node = edge_ref.target();
                break;
            }
        }

        if end(current_node) {
            return i + 1;
        }
    }

    usize::MAX
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = Input::from_argv()?;
    let mut lines = input.as_str().lines();

    let path = lines.next().unwrap();
    lines.next();

    let mut nodes = HashMap::with_capacity(1000);
    let mut ending_in_a = HashSet::with_capacity(1000);
    let mut graph = DiGraph::<&str, char>::with_capacity(1000, 1000);
    for l in lines {
        let node_name = &l[0..3];
        let left_path = &l[7..10];
        let right_path = &l[12..15];

        let node_idx = *nodes
            .entry(node_name)
            .or_insert_with(|| graph.add_node(node_name));
        let left_node = *nodes
            .entry(left_path)
            .or_insert_with(|| graph.add_node(left_path));
        let right_node = *nodes
            .entry(right_path)
            .or_insert_with(|| graph.add_node(right_path));

        graph.add_edge(node_idx, left_node, 'L');
        graph.add_edge(node_idx, right_node, 'R');
        if node_name.ends_with('A') {
            ending_in_a.insert(node_idx);
        }
    }

    let start_node = *nodes.get("AAA").unwrap();
    let target_node = *nodes.get("ZZZ").unwrap();
    let part1 = path_length(&graph, path, start_node, |node| node == target_node);

    let mut part2 = 0;
    for length in ending_in_a.iter().map(|node_idx| {
        path_length(&graph, path, *node_idx, |node| {
            graph.node_weight(node).unwrap().ends_with('Z')
        })
    }) {
        if part2 == 0 {
            part2 = length;
        } else {
            part2 = lcm(part2, length);
        }
    }

    println!("{}\n{}", part1, part2);

    Ok(())
}
