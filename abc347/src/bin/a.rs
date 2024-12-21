fn main() {
    proconio::input! {
        n: usize,
        k: u32,
        a: [u32; n],
    }
    println!("{}",
        a.iter()
         .filter(|&a| a % k == 0)
         .map(|a| (a / k).to_string())
         .collect::<Vec<_>>()
         .join(" ")
    );
}
