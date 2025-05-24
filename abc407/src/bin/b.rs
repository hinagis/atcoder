fn main() {
    proconio::input! {
        x: i32,
        y: i32
    }
    println!("{}", (1..=6).map(|i| (1..=6).map(|j| i + j >= x || (i - j).abs() >= y).filter(|&f| f).count()).sum::<usize>() as f64 / 36.);
}
