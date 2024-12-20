use std::u32;

fn main() {
    proconio::input! {
        n: usize,
        k: [u32; n]
    }
    let mut m = u32::MAX;
    let t = k.iter().sum::<u32>();
    let mut q = std::collections::VecDeque::new();
    q.push_front((k[0], 0));
    while let Some((s, i)) = q.pop_front() {
        m = m.min(s.max(t - s));
        for j in i + 1..n {
            q.push_back((s + k[j], j));
        }
    }
    println!("{}", m);
}
