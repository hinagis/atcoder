fn main() {
    proconio::input! {
        a: f64,
        b: f64,
        d: f64,
    }
    let d = d * (std::f64::consts::PI / 180f64);
    println!("{:} {}", a * d.cos() - b * d.sin(), b * d.cos() + a * d.sin());
}
