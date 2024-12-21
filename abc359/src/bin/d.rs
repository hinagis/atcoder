use proconio::{input as I, marker::Chars as C};

fn main() {
    I! {
        n: usize,
        k: usize,
        s: C
    }
    let mut q = vec![];
    for (i, &c) in s.iter().enumerate() {
        if c == '?' {
            q.push(i);
        }
    }
    for i in 0..q.len() {
        
        for j in i..i + k / 2 {

        }
    }
    println!("{}", n);
}
