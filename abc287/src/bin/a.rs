use proconio::{input as I};
fn main() {
    I! {n: usize}

    let mut c = 0;
    for _ in 0..n {
        I! {s: String}
        if s == "For" {
            c += 1;
        }
    }
    println!("{}", if c > n / 2 {"Yes"} else {"No"});
}
