fn main() {
    proconio::input! {y: u32}
    println!("{}", 365 + if y % 4 != 0 {0} else {if y % 100 != 0 {1} else if y % 400 != 0 {0} else {1}});
}
