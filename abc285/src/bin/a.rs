fn main() {
    proconio::input! {
        a: u32,
        b: u32
    }
    println!("{}", if a == b / 2 {"Yes"} else {"No"});
}
