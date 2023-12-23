fn main() {
    proconio::input! {
        b: u32,
        g: u32
    }
    println!("{}", if b > g {"Bat"} else {"Glove"});
}
