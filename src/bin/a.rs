fn main() {
    proconio::input! {n: u32}
    println!("{}", match n { 2..=4 => "No", _ => "Yes"});
}
