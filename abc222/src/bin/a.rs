use proconio::{input, fastout, marker::Chars};

#[fastout]
fn main() {
    input! {n: Chars}

    for _ in 0..(4-n.len()) {
        print!("0");
    }
    println!("{}", n.iter().collect::<String>());
}
