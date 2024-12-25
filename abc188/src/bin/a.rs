fn main() {
    proconio::input! {
        x: i32,
        y: i32,
    }

    println!("{}", if (x - y).abs() < 3 {"Yes"} else {"No"});
}
