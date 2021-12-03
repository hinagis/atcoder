fn main() {
    proconio::input! {n: usize}
    println!("AGC{0: >03}", if n >= 42 {n + 1} else {n});
}
