fn main() {
    proconio::input! {
        a: u32,
        b: u32,
    }

    println!("{}", if a == 0 {"Silver"} else if b == 0 {"Gold"} else {"Alloy"});
}
