use proconio::{input as I, fastout as F, marker::Usize1 as U};

#[F]
fn main() {
    I! {
        n: usize,
        q: usize,
    }
    let mut s = 0;
    let mut p = (0..n).collect::<Vec<_>>();
    let mut c = vec![1; n];
    for _ in 0..q {
        I! {t: u8}
        if t == 1 {
            I! {
                u: U,
                v: U
            }
            c[p[u]] -= 1;
            if c[p[u]] == 1 {
                s -= 1;
            }
            p[u] = v;
            c[v] += 1;
            if c[v] == 2 {
                s += 1;
            }
        } else {
            println!("{}", s);
        }
    }
}
