fn main() {
    proconio::input! {
        d: u32,
        t: u32,
        s: u32,
    }

    if t * s >= d {
        println!("Yes");
    } else {
        println!("No");
    }
}
