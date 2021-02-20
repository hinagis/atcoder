use proconio::{input, marker::Chars};

fn main() {
    input! {
        x: Chars,
        m: u128,
    }
    let bb = x.iter().fold(0, |m, &v| (v as u128 - '0' as u128).max(m)) + 1;
    let mut b = 2u128.pow((format!("{:b}", m).len() / x.len()).max(1) as u32 - 1).max(bb);
    while calc(&x, b) <= m {
        b += 1;
    }
    println!("{}", b - bb);
}

fn calc(x: &Vec<char>, b: u128) -> u128 {
    let mut n = 0;
    let mut k = 1;
    for i in 0..x.len() {
        let d = x[x.len() - i - 1] as u128 - '0' as u128;
        n += d * k;
        k *= b;
    }
    n
}