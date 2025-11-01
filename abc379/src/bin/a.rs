use proconio::{input as I, marker::Chars as C};

fn main() {
    I! {n: C}
    println!("{}{}{} {}{}{}", n[1], n[2], n[0], n[2], n[0], n[1]);
}
