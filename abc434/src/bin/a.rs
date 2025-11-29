use proconio::input as I;

fn main() {
    I! {
        w: u32,
        b: u32
    }
    println!("{}", w * 1000 / b + 1);
}
