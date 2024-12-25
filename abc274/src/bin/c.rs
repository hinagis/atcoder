use proconio::{input as I, fastout as F, marker::Usize1 as U};

#[F]
fn main() {
    I! {n: usize}

    let mut h = Vec::<u32>::with_capacity(2 * n + 1);
    h.push(0);
    for _ in 0..n {
        I! {a: U}
        h.push(h[a] + 1);
        h.push(h[a] + 1);
    }

    for &a in &h {
        println!("{}", a);
    }
}
