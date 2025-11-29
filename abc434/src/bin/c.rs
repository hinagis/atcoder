use proconio::{input as I, fastout as F};

#[F]
fn main() {
    I! {t: u32}
    for _ in 0..t {f()}
}

fn f() {
    I! {
        n: usize,
        h: i64,
        tlu: [(i64, i64, i64); n]
    }
    let mut p = (0, h, h);
    for (t, l, u) in tlu {
        p = (t, 1.max(p.1 - (t - p.0)), p.2 + (t - p.0));
        if p.1 > u || p.2 < l {
            println!("No");
            return;
        }
        p.1 = p.1.max(l);
        p.2 = p.2.min(u);
    }
    println!("Yes");
}
