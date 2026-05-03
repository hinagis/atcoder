use proconio::{input as I, marker::Bytes as B};

const M: usize = 998244353;

fn main() {
    I! {s: B}
    let mut c = [0; 3];
    for b in s {
        c[(b - b'a') as usize] = (c.iter().sum::<usize>() + 1) % M;
    }
    println!("{}", c.iter().sum::<usize>() % M);
}
