use proconio::input as I;

fn main() {
    I! {
        h: i32,
        b: i32
    }
    println!("{}", 0.max(h - b));
}
