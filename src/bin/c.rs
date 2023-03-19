use proconio::{input as I, fastout as F};

#[F]
fn main() {
    I! {
        n: usize,
        m: usize,
        a: [u32; n],
        b: [u32; m],
    }
    let mut c = Vec::with_capacity(a.len() + b.len());
    for &a in &a {
        c.push((a, 'A'));
    }
    for &b in &b {
        c.push((b, 'B'));
    }
    c.sort();
    let mut a = Vec::with_capacity(a.len());
    let mut b = Vec::with_capacity(b.len());
    for (i, &(_, t)) in c.iter().enumerate() {
        (if t == 'A' {&mut a} else {&mut b}).push(i + 1);
    }
    println!("{}", a.iter().map(|a| a.to_string()).collect::<Vec<_>>().join(" "));
    println!("{}", b.iter().map(|a| a.to_string()).collect::<Vec<_>>().join(" "));
}
