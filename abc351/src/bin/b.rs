use proconio::{input as I, marker::Chars as C};

fn main() {
    I! {
        n: usize,
        a: [C; n],
        b: [C; n]
    }
    for i in 0..n {
        for j in 0..n {
            if a[i][j] != b[i][j] {
                println!("{} {}", i + 1, j + 1);
                return;
            }
        }
    }
}
