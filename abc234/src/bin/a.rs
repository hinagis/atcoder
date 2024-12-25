fn main() {
    proconio::input! {t: u32}
    println!("{}", f(f(f(t) + t) + f(f(t))));
}

fn f(x: u32) -> u32 {
    x * x + 2 * x + 3
}