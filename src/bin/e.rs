use petgraph::algo::min_spanning_tree;
use petgraph::data::FromElements;
use petgraph::graph::UnGraph;

fn main() {
    proconio::input! {
        _: usize,
        m: usize,
        k: u64,
        g: [(u32, u32, u64); m]
    }
    let g = UnGraph::<(), u64>::from_edges(&g);
    println!("{}", UnGraph::<_, _>::from_elements(min_spanning_tree(&g))
        .edge_weights()
        .fold(0, |s, &w| (s + w) % k)
    );
}

