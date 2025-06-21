use proconio::{input as I, marker::Chars as C};

fn main() {
    I! {
        n: usize,
        s: C,
        a: C
    }
    for i in 0..n {
        if s[i] == 'o' && a[i] == 'o' {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
