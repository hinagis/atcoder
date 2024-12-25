use proconio::{input as I, marker::{Usize1 as U, Chars as C}};
fn main() {
    I! {
        n: usize,
        m: usize,
        s: C,
        c: [U; n],
    }
    let mut d = vec![std::collections::VecDeque::new(); m];
    for i in 0..n {
        d[c[i]].push_back(s[i]);
    }
    for q in &mut d {
        q.rotate_right(1);
    }
    let mut t = String::with_capacity(n);
    for c in c {
        t.push(d[c].pop_front().unwrap());
    }
    println!("{}", t);
}
