fn main() {
    proconio::input! {
        n: usize,
        t: [usize]
    }
    let mut d = vec![true; n];
    for &t in &t {
        d[t - 1] ^= true;
    }
    println!("{}", d.iter().filter(|&&f| f).count());
}
