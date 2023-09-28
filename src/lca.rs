use std::cmp::max;

pub struct LCA {
    tin: Vec<i32>,
    tout: Vec<i32>,
    up: Vec<Vec<usize>>,
}
impl LCA {
    pub fn compute(graph: &Vec<Vec<usize>>, root: usize) -> Self {
        let n = graph.len();
        let mut tin = vec![0; n];
        let mut tout = vec![0; n];
        let mut time = 0;
        fn dfs1(
            u: usize,
            p: usize,
            graph: &Vec<Vec<usize>>,
            tin: &mut Vec<i32>,
            tout: &mut Vec<i32>,
            time: &mut i32,
        ) -> usize {
            let mut max_child_height = 0;
            *time += 1;
            tin[u] = *time;
            for v in &graph[u] {
                if *v != p {
                    max_child_height = max(max_child_height, dfs1(*v, u, graph, tin, tout, time));
                }
            }
            *time += 1;
            tout[u] = *time;
            max_child_height + 1
        }
        let k = dfs1(root, root, graph, &mut tin, &mut tout, &mut time).ilog2() as usize + 1;
        let mut up = vec![vec![0; n]; k];
        fn dfs2(u: usize, p: usize, graph: &Vec<Vec<usize>>, up: &mut Vec<Vec<usize>>) {
            up[0][u] = p;
            for v in &graph[u] {
                if *v != p {
                    dfs2(*v, u, graph, up);
                }
            }
        }
        dfs2(root, root, graph, &mut up);
        for i in 1..k {
            for j in 0..n {
                up[i][j] = up[i - 1][up[i - 1][j]];
            }
        }
        LCA { tin, tout, up }
    }
    pub fn lca(&self, u: usize, v: usize) -> usize {
        if self.is_ancestor(u, v) {
            return u;
        }
        if self.is_ancestor(v, u) {
            return v;
        }
        let mut ancestor = u;
        for i in (0..self.up.len()).rev() {
            if !self.is_ancestor(self.up[i][ancestor], v) {
                ancestor = self.up[i][ancestor];
            }
        }
        ancestor
    }
    pub fn is_ancestor(&self, u: usize, v: usize) -> bool {
        self.tin[u] <= self.tin[v] && self.tout[u] >= self.tout[v] // if u is ancestor of v
    }
}
