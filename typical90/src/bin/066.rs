fn main() {
    proconio::input! {
        n: usize,
        lr: [(usize, usize); n]
    }
    let mut m = 0.;
    for i in 0..n {
        let (l, r) = lr[i];
        let a = r - l + 1;
        for j in i + 1..n {
            let (u, v) = lr[j];
            let b = v - u + 1;
            let t = (a * b) as f64;
            if r <= u {continue;}
            if l > v {
                m += 1.;
                continue;
            }

            let (r, c) = if r > v {(v, b * (r - v))} else {(r, 0)};
            let (l, d) = if l > u {(l, (r - l + 1) * (l - u))} else {(u, 0)};
            let s = (r - l + 1) * (r - l) / 2;
            m += (c + d + s) as f64 / t;
        }
    }

    println!("{}", m);
}
