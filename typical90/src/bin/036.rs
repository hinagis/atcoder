use proconio::{input as I, fastout as F, marker::Usize1 as U};

#[F]
fn main() {
    I! {
        n: usize,
        q: usize,
        xy: [(i64, i64); n]
    }

    let mut m = [std::i64::MIN; 4];
    for &(x, y) in &xy {
        let (p, n) = (x + y, x - y);
        m[0] = m[0].max(p);
        m[1] = m[1].max(n);
        m[2] = m[2].max(-n);
        m[3] = m[3].max(-p);
    }

    for _ in 0..q {
        I! {i: U}
        let (x, y) = xy[i];
        let (p, n) = (x + y, x - y);
        println!("{}", (m[0] - p)
                   .max(m[1] - n)
                   .max(m[2] + n)
                   .max(m[3] + p)
        );
    }
}
