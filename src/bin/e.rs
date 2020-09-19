use proconio::input;

fn main() {
    input! { n: usize, k: i64 }

    let mut g = MinCostFlow::new(n * 2 + 2);
    let s = n * 2;
    let t = s + 1;
    const M: i64 = 1000000000;

    g.add_edge(s, t, i64::max_value(), M);
    for i in 0..n {
        g.add_edge(s, i, k, 0);
        g.add_edge(i + n, t, k, 0);
        for j in 0..n {
            input!{ a: i64 }
            g.add_edge(i, j + n, 1, M - a);
        }
    }

    let kn = k * n as i64;
    let v = kn * M - g.flow(s, t, kn);

    let mut f = vec![vec!['.'; n]; n];
    for i in 0..n {
        for &e in &g.edges[i] {
            if e.v == 0 && e.to < 2 * n {
                let j = e.to - n;
                f[i][j] = 'X';
            }
        }
    }

    println!("{}", v);
    println!("{}", f.iter()
        .map(|f| f.iter().collect())
        .collect::<Vec<String>>()
        .join("\n")
    );
}

use std::collections::VecDeque;

#[derive(Copy, Clone, Debug)]
struct Edge {
    to: usize,
    rev: usize,
    v: i64,
    cost: i64,
}

struct MinCostFlow {
    edges: Vec<Vec<Edge>>,
}

impl MinCostFlow {
    fn new(n: usize) -> Self {
        Self {
            edges: vec![vec![]; n],
        }
    }

    fn add_edge(&mut self, s: usize, t: usize, v: i64, cost: i64) {
        let sid = self.edges[s].len();
        let tid = self.edges[t].len();
        self.edges[s].push(Edge { to: t, rev: tid, v, cost });
        self.edges[t].push(Edge { to: s, rev: sid, v: 0, cost: -cost });
    }

    fn flow(&mut self, s: usize, t: usize, mut flow: i64) -> i64 {
        let n = self.edges.len();
        let mut ans = 0;
        let mut from = vec![(0, 0); n];

        while flow > 0 {
            let mut dist = vec![i64::max_value(); n];
            let mut done = vec![false; n];

            dist[s] = 0;

            let mut q = VecDeque::new();
            q.push_back(s);
            while let Some(s) = q.pop_front() {
                done[s] = false;
                for (i, &e) in self.edges[s].iter().enumerate() {
                    if e.v > 0 {
                        let cost = dist[s] + e.cost;
                        if cost < dist[e.to] {
                            dist[e.to] = cost;
                            from[e.to] = (s, i);
                            if !done[e.to] {
                                done[e.to] = true;
                                q.push_back(e.to);
                            }
                        }
                    }
                }
            }

            if dist[t] == i64::max_value() {
                return dist[t];
            }

            let mut v = flow;
            let mut to = t;
            while to != s {
                let (f, i) = from[to];
                v = v.min(self.edges[f][i].v);
                to = f;
            }
            flow -= v;

            ans += dist[t] * v;
            let mut to = t;
            while to != s {
                let (f, i) = from[to];
                let mut edge = &mut self.edges[f][i];
                edge.v -= v;
                let rev = edge.rev;
                self.edges[to][rev].v += v;
                to = f;
            }
        }
        ans
    }
}
