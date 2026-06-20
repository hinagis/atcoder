use itertools::Itertools;
use proconio::{input as I, fastout as F};

#[F]
fn main() {
    I! {
        n: usize,
        hl: [(u64, u64); n],
        q: usize,
        t: [u64; q]
    }
    let mut c: Vec<(u64, u64)> = Vec::with_capacity(n);
    for i in 0..n {
        let mut f = true;
        while let Some(e) = c.pop() {
            if hl[i].0 == e.0 {
                break;
            }
            if hl[i].0 < e.0 {
                c.push(e);
                if hl[i].1 <= e.1 {
                    f = false;
                }
                break;
            }
        }
        if f {
            c.push(hl[i]);
        }
    }
    let t = t.iter().enumerate().sorted_by(|a, b| a.1.cmp(&b.1)).collect_vec();
    let mut a = Vec::with_capacity(q);
    let mut j = 0;
    for i in 0..q {
        while c[j].1 <= *t[i].1 {
            j += 1;
        }
        a.push((t[i].0, c[j].0));
    }
    println!("{}", a.iter().sorted_by(|a, b| a.0.cmp(&b.0)).map(|(_, h)| h).join("\n"));
}
