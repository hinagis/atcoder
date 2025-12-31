use proconio::{input as I, fastout as F, marker::Usize1 as U};

#[F]
fn main() {
    I! {
        n: usize,
        q: usize,
        a: [U; n]
    }
    let mut d = vec![[(0, 0); 30]; n];
    for i in 0..n {
        d[i][0] = (a[i], i + 1);
    }
    for j in 0..29 {
        for i in 0..n {
            let k = d[i][j].0;
            d[i][j + 1] = (d[k][j].0, d[i][j].1 + d[k][j].1);
        }
    }
    for _ in 0..q {
        I! {
            t: usize,
            b: U
        }
        let mut i = b;
        let mut c = 0;
        for j in 0..30 {
            if t & 1 << j != 0 {
                c += d[i][j].1;
                i = d[i][j].0;
            }
        }
        println!("{}", c);
    }
}
