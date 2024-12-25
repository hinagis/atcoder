fn main() {
    proconio::input! {
        n: usize,
        a: [u64; n]
    }
    let s = a.iter().fold(0, |s, a| s + a);
    let mut r = n;
    let mut b = s;
    for l in 0..n {
        while b * 10 > s {
            r -= 1;
            b -= a[r % n];
        }
        while b * 10 < s {
            b += a[r % n];
            r += 1;
        }
        if b * 10 == s {
            println!("Yes");
            return;
        }
        b -= a[l];
    }
    println!("No");
}
