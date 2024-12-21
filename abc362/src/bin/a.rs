fn main() {
    proconio::input! {
        r: u32,
        g: u32,
        b: u32,
        c: String,
    }
    println!("{}", match c.as_str() {
        "Red" => g.min(b),
        "Blue" => g.min(r),
        _ => r.min(b),
    });
}
