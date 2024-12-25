fn main() {
    proconio::input! {
        x: f64,
        y: f64,
    }
    let d = (x * x + y * y).sqrt();
    println!("{} {}", x / d, y / d);
}
