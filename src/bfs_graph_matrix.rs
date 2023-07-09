type Matrix = Vec<usize>;
type WeightedAdjacencyMatrix = Vec<Matrix>;

fn bfs(graph: &WeightedAdjacencyMatrix, source: usize, needle: usize) -> Option<Matrix> {
    let mut seen = vec![false; graph.len()];
    let mut prev: Vec<Option<usize>> = vec![None; graph.len()];
    seen[source] = true;
    let mut queue = vec![source];

    loop {
        let curr = queue.remove(0);
        if curr == needle {
            break;
        }

        let adjs = &graph[curr];
        for i in 0..adjs.len() {
            if adjs[i] == 0 {
                continue;
            }

            if seen[i] {
                continue;
            }

            seen[i] = true;
            prev[i] = Some(curr);
            queue.push(i);
        }

        if queue.is_empty() {
            break;
        }
    }

    let mut curr = needle;
    let mut out: Matrix = vec![];

    while let Some(new_curr) = prev[curr] {
        out.push(curr);
        curr = new_curr;
    }

    if !out.is_empty() {
        let reversed_out: Matrix = out.into_iter().rev().collect();
        let mut result: Matrix = vec![source];
        result.extend(reversed_out);
        return Some(result);
    }

    None
}

#[cfg(test)]
mod test_bfs_graph_matrix {
    use super::*;

    #[test]
    fn it_works() {
        //     >(1)<--->(4) ---->(5)
        //    /          |       /|
        // (0)     ------|------- |
        //    \   v      v        v
        //     >(2) --> (3) <----(6)
        let matrix2: WeightedAdjacencyMatrix = vec![
            vec![0, 3, 1, 0, 0, 0, 0], // 0
            vec![0, 0, 0, 0, 1, 0, 0],
            vec![0, 0, 7, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 1, 0, 5, 0, 2, 0],
            vec![0, 0, 18, 0, 0, 0, 1],
            vec![0, 0, 0, 1, 0, 0, 1],
        ];

        let correct: Matrix = vec![0, 1, 4, 5, 6];

        assert_eq!(bfs(&matrix2, 0, 6), Some(correct));
        assert_eq!(bfs(&matrix2, 6, 0), None);
    }
}
