fn main() {
    proconio::input! {
        n: u64,
        k: u64,
        a: [u64; n],
    }

    let mut l = 0;
    let mut r = *a.iter().max().unwrap();
    while l < r - 1 {
        let m = (l + r) / 2;
        if a.iter().map(|&x| (x - 1) / m).sum::<u64>() <= k {
            r = m;
        } else {
            l = m;
        }
    }
    println!("{}", r);
}
