fn main() {
    proconio::input! {
        n: u32,
        a: u32,
        x: u32,
        y: u32,
    }
    println!("{}", if n > a {x * a + (n - a) * y} else {x * n});

}
