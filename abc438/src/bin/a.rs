use proconio::input as I;

fn main() {
    I! {
        d: i32,
        f: i32,
    }
    println!("{}", 7 - ((d - f) % 7));
}
