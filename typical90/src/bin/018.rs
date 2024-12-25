use std::f64::consts::PI;

fn main() {
    proconio::input! {
        t: u32,
        l: f64, tx: f64, ty: f64,
        q: usize,
        e: [u32; q]
    }
    for &e in &e {
        let d = 2f64 * PI * ((e % t) as f64 / t as f64);
        let y = l * (d + PI / 2f64).cos() / 2f64;
        let z = l * ((d - PI / 2f64).sin() + 1f64) / 2f64;
        let dxy = (tx.powf(2f64) + (ty - y).powf(2f64)).sqrt();
        println!("{}", 180f64 * dxy.atan2(-z) / PI - 90f64);
    }
}
