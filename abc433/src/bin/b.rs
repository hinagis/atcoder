use proconio::{input as I, fastout as F};

#[F]
fn main() {
    I! {
        n: usize,
        a: [u32; n]
    }
    for i in 0..n {
        let mut f = true;
        for j in (0..i).rev() {
            if a[i] < a[j] {
                f = false;
                println!("{}", j + 1);
                break;
            }
        }
        if f {
            println!("-1");
        }
    }
}
