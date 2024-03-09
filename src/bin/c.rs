use proconio::{input as I, fastout as F};
use std::collections::HashSet as H;

#[F]
fn main() {
    I! {
        a: [u32],
        b: [u32],
        c: [u32],
        x: [u32],
    }
    let mut w = H::new();
    for &a in &a {
        for &b in &b {
            w.insert(a + b);
        }
    }
    let mut t = H::new();
    for &w in &w {
        for &c in &c {
            t.insert(w + c);
        }
    }
    for &x in &x {
        println!("{}", if t.contains(&x) {"Yes"} else {"No"});
    }
}
