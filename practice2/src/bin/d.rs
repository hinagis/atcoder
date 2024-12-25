use proconio::{
    fastout,
    input,
    marker::Chars
};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        mut s: [Chars; n]
    }

    let start = n * m;
    let term = start + 1;
    let edge = |i, j| i * m + j;
    let pos = |e| (e / m, e % m);
    let mut g = Dinic::new(n * m + 2);
    for i in 0..n {
        for j in 0..m {
            if s[i][j] != '#' {
                if (i + j) % 2 == 0 {
                    g.add_edge(start, edge(i, j), 1);
                    if i     > 0 && s[i - 1][j] != '#' { g.add_edge(edge(i, j), edge(i - 1, j), 1) }
                    if i + 1 < n && s[i + 1][j] != '#' { g.add_edge(edge(i, j), edge(i + 1, j), 1) }
                    if j     > 0 && s[i][j - 1] != '#' { g.add_edge(edge(i, j), edge(i, j - 1), 1) }
                    if j + 1 < m && s[i][j + 1] != '#' { g.add_edge(edge(i, j), edge(i, j + 1), 1) }
                } else {
                    g.add_edge(edge(i, j), term, 1);
                }
            }
        }
    }

    println!("{}", g.calc(start, term, usize::max_value()));

    for &e in &g.edges[start] {
        if e.cap == 0 {
            let (i, j) = pos(e.to);
            for &ne in &g.edges[e.to] {
                if ne.cap == 0 {
                    let (ni, nj) = pos(ne.to);
                    if i == ni {
                        s[i][j.min(nj)] = '>';
                        s[i][j.max(nj)] = '<';
                    } else {
                        s[i.min(ni)][j] = 'v';
                        s[i.max(ni)][j] = '^';
                    }
                }
            }
        }
    }

    println!("{}",
        s.iter()
         .map(|s| s.iter().collect())
         .collect::<Vec<String>>()
         .join("\n")
    );
}

use num::{Integer, zero, one};
use std::ops::{AddAssign, SubAssign};

#[derive(Copy, Clone)]
struct Edge<T> {
    to: usize,
    rev: usize,
    cap: T,
}

struct Dinic<T> {
    edges: Vec<Vec<Edge<T>>>,
    d: Vec<T>,
    it: Vec<usize>,
}

impl<T: Copy + Clone + Integer + AddAssign + SubAssign> Dinic<T> {
    fn new(n: usize) -> Self {
        Self {
            edges: vec![vec![]; n],
            d: vec![zero(); n],
            it: vec![0; n],
        }
    }

    fn add_edge(&mut self, s: usize, t: usize, cap: T) {
        let rev = self.edges[t].len();
        self.edges[s].push(Edge { to: t, rev, cap });
        let rev = self.edges[s].len() - 1;
        self.edges[t].push(Edge { to: s, rev, cap: zero() });
    }

    fn calc(&mut self, s: usize, t: usize, limit: T) -> T {
        let mut flow = zero();
        while self.calc_distance(s, t) {
            self.it = vec![0; self.it.len()];
            while let Some(f) = self.move_flow(s, t, limit) {
                flow += f;
            }
        }
        flow
    }

    fn calc_distance(&mut self, s: usize, t: usize) -> bool {
        self.d = vec![zero(); self.d.len()];
        self.d[s] = one();
        let mut q = std::collections::VecDeque::new();
        q.push_back(s);
        while let Some(from) = q.pop_front() {
            for &e in &self.edges[from] {
                if e.cap > zero() && self.d[e.to] == zero() {
                    self.d[e.to] = self.d[from] + one();
                    q.push_back(e.to);
                }
            }
        }
        self.d[t] != zero()
    }

    fn move_flow(&mut self, s: usize, t: usize, flow: T) -> Option<T> {
        if s == t {
            Some(flow)
        } else {
            let n = self.edges[s].len();
            for i in self.it[s]..n {
                let e = self.edges[s][i];
                if e.cap > num::zero() && self.d[s] < self.d[e.to] {
                    if let Some(flow) = self.move_flow(e.to, t, e.cap.min(flow)) {
                        self.it[s] = i;
                        self.edges[s][i].cap -= flow;
                        self.edges[e.to][e.rev].cap += flow;
                        return Some(flow)
                    }
                }
            }
            self.it[s] = n;
            None
        }
    }
}
