use proconio::{input as I, marker::Usize1};

fn main() {
    I! {n: usize}
    let mut c = vec![0; n];
    for _ in 0..4 * n - 1 {
        I! {a: Usize1}
        c[a] += 1
    }
    for i in 0..n {
        if c[i] < 4 {
            println!("{}", i + 1);
            return
        }
    }
}
