fn main() {
    proconio::input! {n: u32}
    println!("{}", f(n));
}

fn f(k: u32) -> u32 {
    if k == 0 {
        1
    } else {
        k * f(k - 1)
    }
}