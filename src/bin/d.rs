use proconio::{input as I, marker::Usize1 as U};

fn main() {
    I! {
        n: usize,
        e: [(U, char, U, char)],
    }

    let mut g = petgraph::unionfind::UnionFind::new(n);
    let mut x = 0;
    let mut y = n;
    for (a, _, c, _) in e {
        y -= 1;
        if !g.union(a, c) {
            x += 1;
        }
    }
    println!("{} {}", x, y);
}
