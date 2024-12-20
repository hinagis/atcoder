use itertools::Itertools;

fn main() {
    proconio::input! {s: String}
    let mut p = [0; 26];
    for (i, b) in s.bytes().enumerate() {
        p[(b - b'A') as usize] = i;
    }
    println!("{}", p.iter().tuple_windows().fold(0, |s, (a, b)| s + a.abs_diff(*b)));
}
