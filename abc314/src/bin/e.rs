use proconio::input as I;

fn main() {
    I! {
        n: usize,
        m: usize
    }
    let mut c = Vec::with_capacity(n);
    let mut p = Vec::with_capacity(n);
    let mut z = Vec::with_capacity(n);
    let mut s = Vec::with_capacity(n);

    for _ in 0..n {
        I! {
            d: f64,
            q: usize,
            t: [usize; q]
        }
        c.push(d);
        p.push(q as f64);
        z.push(t.iter().filter(|&&e| e == 0).count() as f64);
        s.push(t);
    }

    let mut e = vec![0f64; m + 10000];
    for b in (0..m).rev() {
        let mut x = std::f64::MAX;
        for i in 0..n {
            x = x.min((c[i] + s[i].iter().fold(0f64, |s, &j| s + e[b + j]) / p[i]) * p[i] / (p[i] - z[i]));
        }
        e[b] = x;
    }
    println!("{}", e[0]);
}
