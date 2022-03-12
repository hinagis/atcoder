use proconio::{input as I, marker::Chars};
fn main() {
    I! {
        n: usize,
        xy: [(u32, u32); n],
        s: Chars
    }
    let mut h = std::collections::HashMap::new();
    for i in 0..n {
        let (x, y) = xy[i];
        let (l, r) = &mut h.entry(y).or_insert((0, u32::max_value()));
        if s[i] == 'L' {
            *l = (*l).max(x);
        } else {
            *r = (*r).min(x);
        }
    }
    for (_, (l, r)) in &h {
        if l > r {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
