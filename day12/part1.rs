use std::collections::HashMap;

use petgraph::{algo::condensation, prelude::*, visit::IntoNodeReferences};

const DIRECTIONS: [[i64; 2]; 4] = [[0, 1], [1, 0], [0, -1], [-1, 0]];

pub fn solve(input: &str) -> usize {
    let (gr, graph) = parse_input(input);

    let result = graph
        .node_references()
        .map(|(_node_index, node_list)| {
            let area = node_list.len();
            let perimiter = node_list
                .iter()
                .map(|node| 4 - gr.neighbors(*node).count())
                .sum::<usize>();
            area * perimiter
        })
        .sum::<usize>();
    result
}

fn parse_input(
    input: &str,
) -> (
    GraphMap<(i64, i64), (), Undirected>,
    Graph<Vec<(i64, i64)>, (), Undirected, NodeIndex>,
) {
    let map: HashMap<(i64, i64), char> = input
        .trim()
        .chars()
        .fold((HashMap::default(), 0, 0), |(mut map, x, y), c| match c {
            '\n' => return (map, 0, y + 1),
            _ => {
                map.insert((x, y), c);
                (map, x + 1, y)
            }
        })
        .0;

    let mut gr: UnGraphMap<(i64, i64), ()> = UnGraphMap::new();
    for ((x, y), c) in map.iter() {
        let node = gr.add_node((*x, *y));
        for [x1, y1] in DIRECTIONS.iter() {
            let new_node = (x + x1, y + y1);
            if map.get(&new_node).is_some_and(|c2| c == c2) {
                gr.add_edge(node, new_node, ());
            }
        }
    }
    (
        gr.clone(),
        condensation(gr.clone().into_graph::<NodeIndex>(), false),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task1() {
        let example = "AAAA
BBCD
BBCC
EEEC";
        assert_eq!(solve(example), 140);
    }
}
