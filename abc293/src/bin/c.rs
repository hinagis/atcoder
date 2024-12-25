use std::collections::{VecDeque as V, HashSet as H};
fn main() {
    proconio::input! {
        h: usize,
        w: usize,
        a: [[u32; w]; h],
    }
    let mut q = V::new();
    q.push_back((0, 0, H::new()));
    let mut c = 0;
    while let Some((i, j, d)) = q.pop_front() {
        let mut d = d.clone();
        if !d.insert(a[i][j]) {
            continue;
        }
        if i == h - 1 && j == w - 1 {
            c += 1;
            continue;
        }
        if i < h - 1 {
            q.push_back((i + 1, j, d.clone()));
        }
        if j < w - 1 {
            q.push_back((i, j + 1, d.clone()));
        }
    }

    println!("{}", c);
}
