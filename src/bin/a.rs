use std::cmp::Ordering::*;

fn main() {
    proconio::input! {s: [(u32, u32)]}
    let (x, y) = s.iter().fold((0, 0), |s, v| (s.0 + v.0, s.1 + v.1));
    println!("{}", match x.cmp(&y) {
        Greater => "Takahashi",
        Less => "Aoki",
        _ => "Draw"
    });
}
