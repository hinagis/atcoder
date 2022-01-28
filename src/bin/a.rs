use proconio::{input, marker::{Usize1, Chars}};

fn main() {
    input! {
        mut s: Chars,
        a: Usize1,
        b: Usize1
    }

    s.swap(a, b);
    println!("{}", s.iter().collect::<String>());
}
