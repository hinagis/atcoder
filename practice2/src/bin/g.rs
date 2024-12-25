use proconio::{input, fastout};

#[fastout]
fn main() {
    input! { n: usize, m: usize }
    let mut scc = SCC::new(n, m);
    for _ in 0..m {
        input! { a: usize, b: usize }
        scc.add_edge(a, b);
    }
    let g = scc.scc();

    println!("{}", g.len());
    for i in 0..g.len() {
        println!("{} {}",
            g[i].len(),
            g[i].iter().map(|j| j.to_string()).collect::<Vec<_>>().join(" ")
        );
    }
}

// Strongly Connected Component
struct SCC {
    n: usize,
    edge: Vec<(usize, usize)>,
}

impl SCC {
    fn new(n: usize, m: usize) -> Self {
        Self {
            n,
            edge: Vec::with_capacity(m),
        }
    }

    fn add_edge(&mut self, s: usize, t: usize) {
        assert!(s < self.n);
        assert!(t < self.n);
        self.edge.push((s, t));
    }

    pub fn scc(&self) -> Vec<Vec<usize>> {
        let (n, ids) = EdgeToIDs::new(self.n, &self.edge);
        let mut g = vec![vec![]; n];
        for (i, &id) in ids.iter().enumerate() {
            g[id].push(i);
        }
        g
    }
}

struct EdgeToIDs {
    csr: CSR,
    index: usize,
    scc_id: usize,
    min_index: Vec<usize>,
    first_index: Vec<Option<usize>>,
    ids: Vec<Option<usize>>,
    stack: Vec<usize>,
}

impl EdgeToIDs {
    fn new(n: usize, edges: &[(usize, usize)]) -> (usize, Vec<usize>) {
        let mut state = Self {
            csr: CSR::new(n, edges),
            index: 0,
            scc_id: 0,
            min_index: vec![0; n],
            first_index: vec![None; n],
            ids: vec![None; n],
            stack: vec![],
        };

        for i in 0..n {
            if state.first_index[i] == None {
                state.dfs(i);
            }
        }

        (state.scc_id, state.ids.iter().map(|i| state.scc_id - 1 - i.unwrap()).collect::<Vec<_>>())
    }

    fn dfs(&mut self, s: usize) {
        self.min_index[s] = self.index;
        self.first_index[s] = Some(self.index);
        self.index += 1;
        self.stack.push(s);
        for i in self.csr.start[s]..self.csr.start[s + 1] {
            let t = self.csr.list[i];
            if self.first_index[t] != None {
                if self.ids[t] == None {
                    self.min_index[s] = self.min_index[s].min(self.min_index[t]);
                }
            } else {
                self.dfs(t);
                self.min_index[s] = self.min_index[s].min(self.min_index[t]);
            }
        }
        if self.min_index[s] == self.first_index[s].unwrap() {
            while let Some(u) = self.stack.pop() {
                self.ids[u] = Some(self.scc_id);
                if u == s {
                    break;
                }
            }
            self.scc_id += 1;
        }
    }
}

// Compressed Sparse Row
struct CSR {
    start: Vec<usize>,
    list: Vec<usize>,
}

impl CSR {
    fn new(n: usize, edge: &[(usize, usize)]) -> Self {
        let mut start = vec![0; n + 1];
        edge.iter().for_each(|&(s, _)| start[s + 1] += 1);
        for i in 1..=n {
            start[i] += start[i - 1];
        }
        let mut index = start.clone();
        let mut list = vec![0; edge.len()];
        for &(s, t) in edge {
            list[index[s]] = t;
            index[s] += 1;
        }

        Self {
            start,
            list,
        }
    }
}
