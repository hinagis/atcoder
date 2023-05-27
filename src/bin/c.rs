use proconio::{input as I, marker::Chars as C};

fn main() {
    I! {
        _: usize,
        m: usize,
        mut h: i32,
        k: i32,
        s: C,
        xy: [(i32, i32); m]
    }
    let mut c = std::collections::HashSet::new();
    for &e in &xy {
        c.insert(e);
    }
    let mut p = (0, 0);
    for &a in &s {
        h -= 1;
        if h < 0 {
            println!("No");
            return;
        }

        p = match a {
            'R' => (p.0 + 1, p.1),
            'L' => (p.0 - 1, p.1),
            'U' => (p.0, p.1 + 1),
            'D' => (p.0, p.1 - 1),
            _ => (0, 0)
        };
        if h < k && c.contains(&p) {
            h = k;
            c.remove(&p);
        }
    }
    println!("Yes");
}
