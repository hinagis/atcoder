use proconio::{input as I, marker::Usize1 as U};

const M: usize = 1_000_000_007;

fn main() {
    I! {
        n: usize,
        q: [(U, U, U, u64)],
    }

    let mut s = 1;
    for b in 0..60 {
        s = s * (0..1 << n).filter(|&t| {
            q.iter().all(|(x, y, z, w)| {
                t >> x & 1 | t >> y & 1 | t >> z & 1 == w >> b & 1
            })
        }).count() % M;
        
    }
    println!("{}", s);
}
