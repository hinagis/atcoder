fn main() {
    proconio::input! {n: usize}
    let mut p = 0f64;
    for _ in 0..n {
        let mut f = 0f64;
        for j in 1..=6 {
            f += p.max(j as f64) / 6f64;
        }
        p = f
    }
    println!("{}", p);
}
