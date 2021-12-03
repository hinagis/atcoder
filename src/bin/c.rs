use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        _n: i64,
        a: i64, b: i64,
        p: i64, q: i64,
        r: i64, s: i64,
    }
    let h = (q - p + 1) as usize;
    let w = (s - r + 1) as usize;
    let mut f = vec![vec!['.'; w]; h];

    let ks = (p - a).max(r - b);
    let ke = (q - a).min(s - b);
    for k in ks..=ke {
        f[(a + k - p) as usize][(b + k - r) as usize] = '#';
    }
    let ks = (p - a).max(b - s);
    let ke = (q - a).min(b - r);
    for k in ks..=ke {
        f[(a + k - p) as usize][(b - k - r) as usize] = '#';
    }

    for i in 0..h {
        println!("{}", f[i].iter().collect::<String>());
    }
}
