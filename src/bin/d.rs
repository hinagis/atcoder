fn main() {
    proconio::input! {
        n: f64,
        x0: f64,
        y0: f64,
        xh: f64,
        yh: f64,
    }

    let a = x0 + (xh - x0) / 2f64;
    let b = y0 + (yh - y0) / 2f64;

    let r = 2f64 * std::f64::consts::PI / n;

    let x1 = r.cos() * (x0 - a) - r.sin() * (y0 - b) + a;
    let y1 = r.sin() * (x0 - a) + r.cos() * (y0 - b) + b;
    println!("{} {}", x1, y1);
}
