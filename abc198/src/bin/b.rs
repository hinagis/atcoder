use proconio::{input, marker::Chars};

fn main() {
    input! {n: Chars}

    let mut s = n.len();
    for i in 0..n.len() {
        if n[i] != '0' {
            s = i;
            break;
        }
    }
    if s == n.len() {
        println!("Yes");
        return;
    }
    let mut t = n.len();
    for i in (0..n.len()).rev() {
        if n[s] == n[i] {
            t = i;
            break;
        }
        if n[i] != '0' {
            println!("No");
            return;
        }
    }
    if s > n.len() - 1 - t {
        println!("No");
        return;
    }
    while s < t {
        if n[s] != n[t] {
            println!("No");
            return;
        }
        s += 1;
        t -= 1;
    }
    println!("Yes");
}
