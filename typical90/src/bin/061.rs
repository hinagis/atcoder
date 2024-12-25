use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {q: usize}
    let mut d = std::collections::VecDeque::new();
    for _ in 0..q {
        input! {
            t: u8,
            x: usize
        }
        if t == 1 {
            d.push_front(x)
        } else if t == 2 {
            d.push_back(x)
        } else {
            println!("{}", d[x - 1])
        }
    }
}
