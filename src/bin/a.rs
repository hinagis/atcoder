fn main() {
    proconio::input! {k: u32}
    println!("2{}:{:02}", k / 60 + 1, k % 60);
}
