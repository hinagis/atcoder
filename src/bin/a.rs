fn main() {
    proconio::input! {
        a: i32,
        b: i32,
        c: i32,
    }

    let mut x = 1;
    while c * x < a {
        x += 1;
    }
    println!("{}", if c * x <= b {c * x} else {-1});
}
