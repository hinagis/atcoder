use proconio::{input as I, marker::Chars as C};
fn main() {
    I! {
        n: usize,
        m: usize,
        s: [C; n]
    }

    let mut t = 0u32;
    for j in 0..m {
        t |= 1 << j;
    }

    let mut a = vec![0u32; n];
    for i in 0..n {
        for j in 0..m {
            if s[i][j] == 'o' {
                a[i] |= 1 << j;
            }
        }
    }

    let mut p = 0;
    for i in 0..n {
        for j in i + 1..n {
            if a[i] | a[j] == t {
                p += 1;
            }
        }
    }
    println!("{}", p);
}
