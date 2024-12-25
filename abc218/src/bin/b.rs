use proconio::{input, marker::Usize1};

fn main() {
    input! {p: [Usize1; 26]}
    println!("{}", p.iter().map(|&c| (b'a' + c as u8) as char).collect::<String>());
}
