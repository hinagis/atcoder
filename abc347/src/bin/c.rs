fn main() {
    proconio::input! {
        n: usize,
        a: u64,
        b: u64,
        mut d: [u64; n],
    }
    let c = a + b;
    d.iter_mut().for_each(|x| *x %= c);
    d.sort();
    d.dedup();
    d.push(d[0]);
    println!("{}", if d.windows(2).any(|w| (w[0] + c - w[1]) % c < a) {"Yes"} else {"No"});
}
