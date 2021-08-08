use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        _h: usize,
        _w: usize,
        n: usize,
        ab: [(usize, usize); n]
    }
    let mut a = vec![(0, 0); n];
    let mut b = vec![(0, 0); n];
    for i in 0..n {
        a[i] = (ab[i].0, i);
        b[i] = (ab[i].1, i);
    }
    a.sort();
    b.sort();
    let mut c = 0;
    let mut d = 0;
    let mut pa = 0;
    let mut pb = 0;
    for i in 0..n {
        if a[i].0 > pa {c += 1}
        if b[i].0 > pb {d += 1}
        pa = a[i].0;
        pb = b[i].0;
        a[i] = (c, a[i].1);
        b[i] = (d, b[i].1);
    }
    a.sort_by(|a, b| a.1.cmp(&b.1));
    b.sort_by(|a, b| a.1.cmp(&b.1));

    for i in 0..n {
        let (c, d) = (a[i].0, b[i].0);
        println!("{} {}", c, d);
    }
}
