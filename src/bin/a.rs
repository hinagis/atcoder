fn main() {
    proconio::input! {
        a: u32,
        b: u32
    }
    println!("{}", if a % 3 != 0 && a + 1 == b {"Yes"} else {"No"});
}
