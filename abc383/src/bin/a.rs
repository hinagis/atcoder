fn main() {
    proconio::input! {
        n: usize,
        tv: [(u32, u32); n]
    }
    let mut v = tv[0].1;
    for i in 1..n {
        let d = tv[i].0 - tv[i - 1].0;
        v = v.saturating_sub(d) + tv[i].1;
    }
    println!("{}", v);
}
