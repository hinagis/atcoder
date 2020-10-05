fn main() {
    proconio::input! {
        a: i32,
        b: i32,
    }
    let x = (a + b) / 2;
    let y = (a - b) / 2;
    println!("{} {}", x, y);
}
