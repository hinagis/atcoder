fn main() {
    proconio::input! {
        n: usize,
        a: u64,
        b: u64,
        mut d: [u64; n],
    }
    for i in 0..n {
        d[i] %= a + b;
    }
    d.sort();
    println!("{}", if d[n - 1] - d[0] < a {"Yes"} else {"No"});
}
