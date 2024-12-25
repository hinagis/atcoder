use proconio::{input as I, marker::Chars as C};
fn main() {
    I! {
        n: usize,
        a: [C; n]
    }

    for i in 0..n {
        for j in 0..i {
            if a[i][j] == 'W' && a[j][i] != 'L' ||
               a[i][j] == 'L' && a[j][i] != 'W' ||
               a[i][j] == 'D' && a[j][i] != 'D' {
                println!("incorrect");
                return;
            }
        }
    }
    println!("correct");
}
