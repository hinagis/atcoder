use proconio::{input, fastout};

#[fastout]
fn main() {
    input!{
        a: i32,
        b: i32,
        h: i32,
        m: i32,
    }
    let a = a as f64;
    let b = b as f64;
    let h = h as f64;
    let m = m as f64;

    let (h, m) = (h / 12_f64, m / 60_f64);
    let h = 360_f64 * h + (360_f64 / 12_f64) * m;
    let m = 360_f64 * m;
    let (h, m) = (h.to_radians(), m.to_radians());
    let (hx, hy) = (a * h.cos(), a * h.sin());
    let (mx, my) = (b * m.cos(), b * m.sin());
    let (dx, dy) = ((hx - mx).abs(), (hy - my).abs());

    println!("{}", (dx.powf(2_f64) + dy.powf(2_f64)).sqrt());
}
