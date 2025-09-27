fn main() {
    proconio::input! {n: i32}
    println!("{}", (1..=n).fold(0, |s, i| s + (if i % 2 == 0 {1} else {-1}) * i * i * i));
}
