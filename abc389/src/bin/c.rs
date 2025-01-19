use proconio::{input as I, fastout as F};

#[F]
fn main() {
    I! {q: usize}
    let mut head = 0;
    let mut tail = 0;
    let mut snakes = Vec::with_capacity(3 * 10usize.pow(5) + 1);
    snakes.push(0);
    for _ in 0..q {
        I! {t: u8}
        match t {
            1 => {
                I! {l: u64}
                snakes.push(snakes[tail] + l);
                tail += 1;
            }
            2 => {
                head += 1;
            }
            _ => {
                I! {k: usize}
                println!("{}", snakes[head + k - 1] - snakes[head]);
            }
        }
    }
}
