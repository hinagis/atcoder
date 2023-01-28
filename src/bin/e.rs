use proconio::{input as I, fastout as F, marker::Bytes as B};

#[F]
fn main() {
    I! {
        n: usize,
        s: [B; n]
    }

    let mut p = (0..n).collect::<Vec<_>>();
    p.sort_by_key(|&x| &s[x]);

    let mut r = vec![0; n];
    for i in 0..n - 1 {
        let (i, j) = (p[i], p[i + 1]);
        let v = s[i]
            .iter()
            .zip(s[j].iter())
            .take_while(|&(&a, &b)| a == b)
            .count();
        r[i] = r[i].max(v);
        r[j] = r[j].max(v);
    }

    for c in r {
        println!("{}", c);
    }
}
