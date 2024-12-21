use proconio::{input as I, marker::Chars as C};

fn main() {
    I! {
        n: usize,
        s: C
    }

    let calc = || {
        if n % 2 == 0 || s[n / 2] != '/' {
            return false;
        }
        for i in 0..n / 2 {
            if s[i] != '1' || s[i + n / 2 + 1] != '2' {
                return false;
            }
        }
        true
    };
    println!("{}", if calc() {"Yes"} else {"No"});
}
