use itertools::Itertools;
use proconio::{input as I, fastout as F, marker::Usize1 as U};

#[F]
fn main() {
    I! {
        n: usize,
        q: [(u8, U, U, i64)]
    }

    let mut d = vec![0; n];
    for (_, x, _, v) in q.iter().filter(|e| e.0 == 0).sorted() {
        d[x + 1] = v - d[*x];
    }
 
    let mut g = petgraph::unionfind::UnionFind::new(n);
    for (t, x, y, v) in q {
        if t == 0 {
            g.union(x, y);
        } else {
            if g.find(x) != g.find(y) {
                println!("Ambiguous");
            }else {
                println!("{}", d[y] + (v - d[x]) * if (x.max(y) - x.min(y)) % 2 == 0 {1} else {-1});
            }
        }
    }
}
