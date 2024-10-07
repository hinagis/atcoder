fn main() {
    proconio::input! {s: [String; 12]}
    println!("{}", s.iter().enumerate().filter(|&(i, s)| i + 1 == s.len()).count());
}
