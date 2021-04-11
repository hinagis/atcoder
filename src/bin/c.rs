fn main() {
    proconio::input! {
        r: f64,
        x: f64,
        y: f64,
    }
    let g = (x * x + y * y).sqrt();
    println!("{}", if g == r {
            1
        } else if g <= 2f64 * r {
            2
        } else {
            (g / r).ceil() as u64
        }
    );
}
