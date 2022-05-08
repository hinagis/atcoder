use proconio::{input as I, fastout};

#[fastout]
fn main() {
    I! {
        n: usize,
        a: usize,
        b: usize,
    }
    let mut s = (String::new(), String::new());
    let mut c = ('.', '#');
    for _ in 0..n {
        for _ in 0..b {
            s.0.push(c.0);
            s.1.push(c.1);
        }
        c = (c.1, c.0);
    }
    for _ in 0..n {
        for _ in 0..a {
            println!("{}", s.0);
        }
        s = (s.1, s.0);
    }
}
