use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: Chars,
    }
    let n: Vec<usize> = n.iter().map(|&c| c as usize - '0' as usize).collect();
    let sum = n.iter().fold(0, |s, i| s + i);
    if sum >= 3 {
        let p = sum % 3;
        if p == 0 {
            println!("0");
        } else {
            for &i in &n {
                if sum - i >= 3 && i % 3 == p {
                    println!("1");
                    return;
                }
            }
            for i in 1..n.len() {
                if sum - (n[i] + n[i - 1]) >= 3 && (n[i] + n[i - 1]) % 3 == p {
                    println!("2");
                    return;
                }
            }
            println!("-1");
        }
    } else {
        println!("-1");
    }
}
