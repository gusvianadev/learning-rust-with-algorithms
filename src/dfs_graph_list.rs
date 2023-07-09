use crate::stuff::{MatrixValue, WeightedAdjacencyList};

fn walk(
    graph: &WeightedAdjacencyList,
    curr: MatrixValue,
    needle: MatrixValue,
    seen: &mut Vec<bool>,
    path: &mut Vec<MatrixValue>,
) -> bool {
    if seen[curr] {
        return false;
    }

    seen[curr] = true;

    // RECURSE
    // pre
    path.push(curr);
    if curr == needle {
        return true;
    }
    // recurse
    let list = &graph[curr];

    for i in 0..list.len() {
        let edge = &list[i];

        if walk(graph, edge.to, needle, seen, path) {
            return true;
        }
    }

    // post
    path.pop();

    false
}

fn dfs(
    graph: &WeightedAdjacencyList,
    source: MatrixValue,
    needle: MatrixValue,
) -> Option<Vec<MatrixValue>> {
    let mut seen = vec![false; graph.len()];
    let mut path: Vec<MatrixValue> = vec![];

    walk(graph, source, needle, &mut seen, &mut path);

    if path.is_empty() {
        return None;
    }

    Some(path)
}

#[cfg(test)]
mod test_dfs_graph_list {
    use crate::stuff::GraphEdge;

    use super::*;

    #[test]
    fn it_works() {
        //     >(1)<--->(4) ---->(5)
        //    /          |       /|
        // (0)     ------|------- |
        //    \   v      v        v
        //     >(2) --> (3) <----(6)
        let list2: WeightedAdjacencyList = vec![
            vec![
                GraphEdge { to: 1, weight: 3 },
                GraphEdge { to: 2, weight: 1 },
            ],
            vec![GraphEdge { to: 4, weight: 1 }],
            vec![GraphEdge { to: 3, weight: 7 }],
            vec![],
            vec![
                GraphEdge { to: 1, weight: 1 },
                GraphEdge { to: 3, weight: 5 },
                GraphEdge { to: 5, weight: 2 },
            ],
            vec![
                GraphEdge { to: 2, weight: 18 },
                GraphEdge { to: 6, weight: 1 },
            ],
            vec![GraphEdge { to: 3, weight: 1 }],
        ];

        assert_eq!(dfs(&list2, 0, 6), Some(vec![0, 1, 4, 5, 6]));
        assert_eq!(dfs(&list2, 6, 0), None);
    }
}
