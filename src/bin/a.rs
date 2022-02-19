fn main() {
    proconio::input! {h: f64}
    println!("{}", (h * (12800000f64 + h)).sqrt());
}
