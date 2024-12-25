fn main() {
    proconio::input! {
        n: usize,
        s: u32,
        d: u32,
        xy: [(u32, u32); n]
    }

    for &(x, y) in &xy {
        if x < s && y > d {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
