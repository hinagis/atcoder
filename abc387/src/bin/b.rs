fn main() {
    proconio::input! {x: usize}
    println!("{}", (1..=9).map(|i| (1 * i..=9 * i).step_by(i)).flatten().filter(|&n| n != x).sum::<usize>());
}
