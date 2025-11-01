use proconio::{input as I, marker::Chars as C};

fn main() {
    I! {
        n: usize,
        k: usize,
        s: C
    }
    let mut o = 0;
    let mut c = 0;
    for i in 0..n {
        if s[i] == 'O' {
            o += 1;
            if o >= k {
                o = 0;
                c += 1;
            }
        } else {
            o = 0;
        }
    }
    println!("{}", c);
}
