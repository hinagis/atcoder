use proconio::{input as I};
fn main() {
    I! {n: usize}

    let mut e = Vec::with_capacity(n);
    let mut o = Vec::with_capacity(n);
    for _ in 0..n {
        I! {a: u32}
        if a % 2 == 0 {
            e.push(a);
        } else {
            o.push(a);
        }
    }
    if e.len() < 2 && o.len() < 2 {
        println!("-1");
    } else {
        println!("{}", (
            if e.len() < 2 {0} else {e.sort_by(|a, b| b.cmp(a)); e[0] + e[1]}
        ).max(
            if o.len() < 2 {0} else {o.sort_by(|a, b| b.cmp(a)); o[0] + o[1]}
        ));
    }
}
