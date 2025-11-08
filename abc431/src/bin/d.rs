use proconio::input as I;

fn main() {
    I! {
        n: usize,
        mut whb: [(usize, u64, u64); n]
    }
    whb.sort_by(|a, b| a.0.cmp(&b.0));
    let mut v = vec![0; whb.iter().fold(0, |s, (w, _, _)| s + w) + 1];
    v[0] = whb.iter().fold(0, |s, (_, h, _)| s + h);
    for i in 0..n {
        let (w, h, b) = whb[i];
        for j in (0..v.len() - w).rev() {
            if v[j] == 0 {continue}
            v[j + w] = v[j + w].max(v[j] - h + b);
        }
    }
    let mut r = 0;
    for w in v.len() / 2..v.len() {
        r = r.max(v[w]);
    }
    println!("{}", r);
}
