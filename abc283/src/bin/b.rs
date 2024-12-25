use proconio::{input as I, fastout as F, marker::Usize1 as U};

#[F]
fn main() {
    I! {
        n: usize,
        mut a: [u32; n],
        q: usize
    }
    for _ in 0..q {
        I! {
            t: u8,
            k: U
        }
        if t == 1 {
            I! {x: u32}
            a[k] = x;
        } else {
            println!{"{}", a[k]}
        }
    }
}
