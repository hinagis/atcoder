fn main() {
    proconio::input! {a: [u32]}
    println!("{}", if a.windows(2).all(|a| a[0] < a[1]) {"Yes"} else {"No"});
}
