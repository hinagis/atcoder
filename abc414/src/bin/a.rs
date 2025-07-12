use proconio::input as I;

fn main() {
    I! {
        n: u32,
        l: u32,
        r: u32
    }
    let mut c = 0;
    for _ in 0..n {
        I! {
            x: u32,
            y: u32,
        }
        if x <= l && y >= r {
            c += 1;
        }
    }
    println!("{}", c);
}
