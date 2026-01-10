use proconio::input as I;

fn main() {
    I! {
        x: u32,
        y: u32
    }
    println!("{}", x * 2u32.pow(y));
}
