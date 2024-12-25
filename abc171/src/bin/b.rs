fn main() {
    proconio::input! {
        n: usize, k: usize,
        mut p: [u32; n],
    }
    p.sort();
    println!("{}", p[0..k].iter().sum::<u32>());
}
