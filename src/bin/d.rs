use proconio::{input as I, marker::Usize1 as U};
use petgraph::algo::dijkstra as A;
use petgraph::graph::UnGraph as G;

fn main() {
    I! {
        n1: usize,
        n2: usize,
        m: usize,
        ab: [(U, U); m]
    }

    let mut g1 = G::<(), f64, _>::new_undirected();
    let ns1: Vec<_> = (0..n1).map(|_| g1.add_node(())).collect();
    g1.extend_with_edges((0..m).filter(|&i| ab[i].0 < n1).map(|i| (ns1[ab[i].0], ns1[ab[i].1], 1.0f64)));
    let p1 = A(&g1, ns1[0], None, |_| 1);
    let m1 = p1.iter().fold(0, |s, (_, &w)| s.max(w));

    let mut g2 = G::<(), f64, _>::new_undirected();
    let ns2: Vec<_> = (n1..n1 + n2).map(|_| g2.add_node(())).collect();
    g2.extend_with_edges((0..m).filter(|&i| ab[i].0 >= n1).map(|i| (ns2[ab[i].0 - n1], ns2[ab[i].1 - n1], 1.0f64)));
    let p2 = A(&g2, ns2[n2 - 1], None, |_| 1);
    let m2 = p2.iter().fold(0, |s, (_, &w)| s.max(w));
    println!("{}", m1 + m2 + 1);
}
