use proconio::{input, fastout, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [Chars; 2 * n]
    }
    let mut b = vec![(0, 0); 2 * n];
    for i in 0..2 * n {
        b[i] = (i, 0);
    }
    for j in 0..m {
        for k in 0..n {
            let (u, v) = (a[b[2 * k].0][j], a[b[2 * k + 1].0][j]);
            if u != v {
                if u == 'G' && v == 'C'
                || u == 'C' && v == 'P'
                || u == 'P' && v == 'G' {
                    b[2 * k].1 += 1;
                } else {
                    b[2 * k + 1].1 += 1;
                }
            }
        }
        b.sort_by(|&a, &b| {let t = b.1.cmp(&a.1); if t == std::cmp::Ordering::Equal {a.0.cmp(&b.0)} else {t}})
    }
    for i in 0..2 * n {
        println!("{}", b[i].0 + 1);
    }
}
