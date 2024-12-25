use std::collections::{VecDeque as Q, HashSet as H};

fn main() {
    proconio::input! {
        n: usize,
        s: (i64, i64),
        t: (i64, i64),
        p: [(i64, i64, i64); n]
    }

    let mut q = Q::new();
    let mut g = H::new();
    let mut f = vec![true; n];
    for i in 0..n {
        let (x, y, r) = p[i];
        if (s.0 - x) * (s.0 - x) + (s.1 - y) * (s.1 - y) == r * r {
            f[i] = false;
            q.push_back(i);
        }
        if (t.0 - x) * (t.0 - x) + (t.1 - y) * (t.1 - y) == r * r {
            g.insert(i);
        }
    }

    while let Some(i) = q.pop_front() {
        if g.contains(&i) {
            println!("Yes");
            return;
        }
        let (a, b, r1) = p[i];
        for j in 0..n {
            let (x, y, r2) = p[j];
            let d = (a - x) * (a - x) + (b - y) * (b - y);
            if f[j] && d <= (r1 + r2) * (r1 + r2) && d >= (r1 - r2) * (r1 - r2) {
                f[j] = false;
                q.push_back(j);
            }
        }
    }
    println!("No");
}
