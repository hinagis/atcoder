use proconio::{input as I, fastout as O};

#[O]
fn main() {
    let mut d = std::collections::VecDeque::new();
    I! {q: usize}
    for _ in 0..q {
        I! {t: u8}
        if t == 1 {
            I! {x: u64, c: u64}
            d.push_back((x, c));
        } else {
            let mut r = 0;
            I! {mut c: u64}
            while c > 0 {
                let f = d.pop_front().unwrap();
                if f.1 > c {
                    r += f.0 * c;
                    d.push_front((f.0, f.1 - c));
                    c = 0;
                } else {
                    r += f.0 * f.1;
                    c -= f.1;
                }
            }
            println!("{}", r);
        }
    }
}
