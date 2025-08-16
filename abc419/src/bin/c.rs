use proconio::input as I;

fn main() {
    I! {n: usize}
    let (mut a, mut b, mut c, mut d) = (u32::MAX, u32::MIN, u32::MAX, u32::MIN);
    for _ in 0..n {
        I! {
            x: u32,
            y: u32,
        }
        a = a.min(x);
        b = b.max(x);
        c = c.min(y);
        d = d.max(y);
    }
    println!("{}", (b - a + 1).max(d - c + 1) / 2);
}
