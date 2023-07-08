use proconio::{input as I, fastout as F, marker::Chars as C};

#[F]
fn main() {
    I! {
        n: usize,
        a: [C; n]
    }
    let t = n - 1;
    let mut b = a.clone();
    for i in 1..n {
        b[0][i] = a[0][i - 1];
        b[t][t - i] = a[t][n - i];
        b[i][t] = a[i - 1][t];
        b[t - i][0] = a[n - i][0];
    }
    for i in 0..n {
        println!("{}", b[i].iter().collect::<String>());
    }
}
