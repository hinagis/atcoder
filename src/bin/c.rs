use proconio::{input as I, fastout as F, marker::Usize1 as U};

#[F]
fn main() {
    I! {
        n: i32,
        q: usize
    }
    let mut d = std::collections::VecDeque::with_capacity(n as usize);
    for i in 1..=n {
        d.push_back((i, 0))
    }
    for _ in 0..q {
        I! {x: u8}
        if x == 1 {
            I! {c: char}
            let f = *d.front().unwrap();
            let p = match c {
                'R' => (f.0 + 1, f.1),
                'L' => (f.0 - 1, f.1),
                'U' => (f.0, f.1 + 1),
                'D' => (f.0, f.1 - 1),
                _ => (0, 0)
            };
            d.pop_back();
            d.push_front(p);
        } else {
            I! {p: U}
            println!("{} {}", d[p].0, d[p].1);
        }
    }
}
