use crate::stuff::WeightedAdjacencyList;

fn has_unvisited(seen: &Vec<bool>, dists: &Vec<usize>) -> bool {
    seen.iter()
        .enumerate()
        .any(|(i, s)| !s && dists[i] < usize::MAX)
}

fn get_lowest_unvisited(seen: &Vec<bool>, dists: &Vec<usize>) -> usize {
    let mut idx = 0;
    let mut lowest_distance = usize::MAX;

    for i in 0..seen.len() {
        if seen[i] {
            continue;
        }

        if lowest_distance > dists[i] {
            lowest_distance = dists[i];
            idx = i;
        }
    }

    idx
}

fn dijkstra_list(source: usize, sink: usize, graph: WeightedAdjacencyList) -> Vec<usize> {
    let mut seen = vec![false; graph.len()];
    let mut prev: Vec<Option<usize>> = vec![None; graph.len()];
    let mut dists = vec![usize::MAX; graph.len()];

    dists[source] = 0;

    while has_unvisited(&seen, &dists) {
        let curr = get_lowest_unvisited(&seen, &dists);

        seen[curr] = true;

        let adjs = &graph[curr];

        for i in 0..adjs.len() {
            let edge = &adjs[i];

            if seen[edge.to] {
                continue;
            }

            let dist = dists[curr] + edge.weight;

            if dist < dists[edge.to] {
                dists[edge.to] = dist;
                prev[edge.to] = Some(curr);
            }
        }
    }

    let mut out: Vec<usize> = vec![];
    let mut curr = sink;

    while let Some(new_curr) = prev[curr] {
        out.push(curr);
        curr = new_curr;
    }

    out.push(source);
    out.into_iter().rev().collect()
}

#[cfg(test)]
mod test_dijkstra_list {
    use crate::stuff::GraphEdge;

    use super::*;

    #[test]
    fn it_works() {
        //      (1) --- (4) ---- (5)
        //    /  |       |       /|
        // (0)   | ------|------- |
        //    \  |/      |        |
        //      (2) --- (3) ---- (6)
        let mut list1: WeightedAdjacencyList = vec![
            vec![
                GraphEdge { to: 1, weight: 3 },
                GraphEdge { to: 2, weight: 1 },
            ],
            vec![
                GraphEdge { to: 0, weight: 3 },
                GraphEdge { to: 2, weight: 4 },
                GraphEdge { to: 4, weight: 1 },
            ],
            vec![
                GraphEdge { to: 1, weight: 4 },
                GraphEdge { to: 3, weight: 7 },
                GraphEdge { to: 0, weight: 1 },
            ],
            vec![
                GraphEdge { to: 2, weight: 7 },
                GraphEdge { to: 4, weight: 5 },
                GraphEdge { to: 6, weight: 1 },
            ],
            vec![
                GraphEdge { to: 1, weight: 1 },
                GraphEdge { to: 3, weight: 5 },
                GraphEdge { to: 5, weight: 2 },
            ],
            vec![
                GraphEdge { to: 6, weight: 1 },
                GraphEdge { to: 4, weight: 2 },
                GraphEdge { to: 2, weight: 18 },
            ],
            vec![
                GraphEdge { to: 3, weight: 1 },
                GraphEdge { to: 5, weight: 1 },
            ],
        ];
        assert_eq!(dijkstra_list(0, 6, list1), [0, 1, 4, 5, 6]);
    }
}
