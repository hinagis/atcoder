fn main() {
    proconio::input! {
        a: u32,
        b: u32,
        c: u32,
    }

    if a == b {
        println!("{}", c);
    } else if b == c {
        println!("{}", a);
    } else if c == a {
        println!("{}", b);
    } else {
        println!("0");
    }
}
