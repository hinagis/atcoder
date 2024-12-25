fn main() {
    proconio::input! {
        a: u32,
        b: u32,
    }
    println!("{}",
        if a + b >= 15 && b >= 8 {
            1
        } else if a + b >= 10 && b >= 3 {
            2
        } else if a + b >= 3 {
            3
        } else {
            4
        }
    );
}
