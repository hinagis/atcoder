fn main() {
    proconio::input! {
        n: usize,
    }
    let mut r = vec![1u128; n];

    for m in 1..n {
        let mut i = m;
        while i < n {
            r[i] += 1;
            i += m + 1;
        }
    }
    for i in 0..n {
        r[i] *= i as u128 + 1;
    }
    let r: u128 = r.iter().sum();
    println!("{}", r);
}
