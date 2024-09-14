use proconio::{input as I, fastout as F};

#[F]
fn main() {
    I! {
        n: usize,
        x: [i64; n]
    }

    let mut s = vec![0; n + 1];
    for i in 0..n {
        I! {p: u64}
        s[i + 1] = s[i] + p;
    }

    I! {q: usize}
    for _ in 0..q {
        I! {
            l: i64,
            r: i64
        }
        let l = x.binary_search(&l);
        let r = x.binary_search(&r);
        println!("{}", s[match r {Ok(r) => r + 1, Err(r) => r}] - s[match l {Ok(l) => l, Err(l) => l}]);
    }
}
