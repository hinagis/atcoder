use proconio::{input as I, marker::Chars as C};

fn main() {
    I! {
        n: usize,
        m: usize,
        s: [C; n],
        t: [C; m]
    }
    for i in 0..n {
        'outer: for j in 0..=n - m {
            for y in 0..m {
                for x in 0..m {
                    if s[i + y][j + x] != t[y][x] {
                        continue 'outer;
                    }
                }
            }
            println!("{} {}", i + 1, j + 1);
            return;
        }
    }
}
