use proconio::{input as I, marker::Usize1 as U};
fn main() {
    I! {
        n: usize,
        k: usize,
        a: [U; k],
        p: [(f64, f64); n]
    }

    let mut r = 0f64;
    for i in 0..n {
        if ! a.contains(&i) {
            let mut m = 1000_000f64;
            for &j in &a {
                let (x, y) = p[j];
                m = m.min(((x - p[i].0).powf(2f64) + (y - p[i].1).powf(2f64)).sqrt());
            }
            r = r.max(m);
        }
    }

    println!("{}", r);
}
