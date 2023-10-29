fn main() {
    proconio::input! {
        x: i32,
        y: i32
    }
    println!("{}", if (x < y && y - x < 3) || (x > y && x - y < 4) {"Yes"} else {"No"});
}
