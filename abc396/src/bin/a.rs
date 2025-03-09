fn main() {
    proconio::input! {a: [u8]}
    println!("{}", if a.windows(3).any(|a| a[0] == a[1] && a[1] == a[2]) {"Yes"} else {"No"});
}
