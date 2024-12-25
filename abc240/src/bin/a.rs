fn main() {
    proconio::input! {
        a: u32,
        b: u32
    }

    println!("{}", if b - a == 1 || (a, b) == (1, 10) {"Yes"} else {"No"});
}
