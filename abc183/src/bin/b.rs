fn main() {
    proconio::input! {
        sx: i32,
        sy: i32,
        gx: i32,
        gy: i32,
    }
    let sx = sx as f64;
    let sy = sy as f64;
    let gx = gx as f64;
    let gy = gy as f64;
    let a = (gy + sy) / (gx - sx);
    let b = sy + (a * sx);
    println!("{}", b / a);
}
