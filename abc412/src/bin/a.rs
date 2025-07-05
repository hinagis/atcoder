fn main() {
    proconio::input! {d: [(u32, u32)]}
    println!("{}", d.iter().filter(|(a, b)| a < b).count());
}
