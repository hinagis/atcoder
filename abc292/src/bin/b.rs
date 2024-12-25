use proconio::{input as I, fastout as F, marker::Usize1 as U};

#[F]
fn main() {
    I! {
        n: usize,
        q: usize
    }

    let mut v = vec![0; n];
    for _ in 0..q {
        I!{
            e: u8,
            x: U
        }
        if e == 3 {
            println!("{}", if v[x] >= 2 {"Yes"} else {"No"});
        } else {
            v[x] += e;
        }
    }
}
