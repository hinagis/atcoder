use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, UnGraph};

fn main() {
    proconio::input! {
        n: usize,
        m: usize,
        e: [(usize, usize, u32); m]
    }
    let g = UnGraph::<usize, u32, usize>::from_edges(&e);
    let mut d = vec![];
    for i in 0..n {
        d.push(dijkstra(&g, (i + 1).into(), None, |e| *e.weight()))
    }
    let mut r = 0;
    for &(a, b, c) in &e {
        let &w = dijkstra(&g, a.into(), Some(b.into()), |e| *e.weight()).get(&NodeIndex::new(b)).unwrap();
        if c > w {
            r += 1
        } else {
            if c == w {
                let mut g = g.clone();
                g.remove_edge(g.find_edge(NodeIndex::new(a), NodeIndex::new(b)).unwrap());
                if let Some(&w) = dijkstra(&g, a.into(), Some(b.into()), |e| *e.weight()).get(&NodeIndex::new(b)) {
                    if c == w {
                        r += 1
                    }
                }
            }
        }
    }
    println!("{}", r);
}
