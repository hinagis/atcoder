fn main() {
    proconio::input! {
        m: usize,
        h: usize,
    }

    println!("{}", if h % m == 0 {"Yes"} else {"No"});
}
