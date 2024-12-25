fn main() {
    proconio::input! {
        n: usize,
        x: usize,
    }
    let c = ((x - 1) / n) as u8;

    println!("{}", (b'A' + c) as char);
}
