use proconio::{input as I, fastout as F};

#[F]
fn main() {
    I! {q: usize}
    let mut w = std::collections::VecDeque::new();
    for _ in 0..q {
        I! {t: u8}
        if t == 1 {
            I! {x: u32}
            w.push_back(x);
        } else {
            println!("{}", w.pop_front().unwrap());
        }
    }
}
