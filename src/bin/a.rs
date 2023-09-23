use proconio::{input as I, marker::Bytes as B};

fn main() {
    I! {n: B}
    let mut d = b'9' + 1;
    for &c in n.iter() {
        if c >= d {
            println!("No");
            return;
        }
        d = c;
    }
    println!("Yes");
}
