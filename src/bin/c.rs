fn main() {
    proconio::input! {
        r1: i32,
        c1: i32,
        r2: i32,
        c2: i32,
    }
    if r1 == r2 && c1 == c2 {
        println!("0");
    } else if (r1 + c1 == r2 + c2)
    || (r1 - c1 == r2 - c2)
    || ((r1 - r2).abs() + (c1 - c2).abs() <= 3) {
        println!("1");
    } else {
        let d = ((r1 - r2).abs() - (c1 - c2).abs()).abs();
        println!("{}", if d <= 3 || d % 2 == 0 {2} else {3});
    }
}
