fn main() {
    proconio::input! {
        n: usize,
        k: i32,
        a: [i32; n],
        b: [i32; n]
    }
    let d = a.iter().zip(b.iter()).fold(0, |d, (a, b)| d + (a - b).abs());
    println!("{}", if k >= d && (k - d) % 2 == 0 {"Yes"} else {"No"});
}
