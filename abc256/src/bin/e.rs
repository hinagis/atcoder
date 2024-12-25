use proconio::{input as I, marker::Usize1 as U};
use petgraph::{Graph as G, algo::kosaraju_scc as S};

fn main() {
    I! {
        n: usize,
        x: [U; n],
        c: [u64; n]
    }
    
    let mut g = G::new();
    let p: Vec<_> = (0..n).map(|i| g.add_node(i)).collect();
    g.extend_with_edges((0..n).map(|i| (p[i], p[x[i]], c[i])));

    let mut r = 0;
    for t in S(&g) {
        if t.len() >= 2 {
            let mut m = u64::max_value();
            for i in t {
                m = m.min(c[g[i]]);
            }
            r += m;
        }
    }
    println!("{}", r);
}
