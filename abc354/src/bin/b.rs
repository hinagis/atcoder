fn main() {
    proconio::input! {mut sc: [(String, usize)]}
    let t = sc.iter().fold(0, |t, (_, c)| t + c);
    sc.sort();
    println!("{}", sc[t % sc.len()].0);
}
