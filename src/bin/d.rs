fn main() {
    proconio::input! {
        n: usize,
        s: u64,
        mut a: [u64; n]
    }
    let t = a.iter().sum::<u64>();
    let s = s - (s / t) * t;
    if s == 0 {
        println!("Yes");
        return;
    }

    a.append(&mut a.clone());
    let mut l = 0;
    let mut r = 0;
    let mut c = 0;
    while l < n * 2 {
        while r < n * 2 && c < s {
            c += a[r];
            r += 1;
        }
        if c == s {
            println!("Yes");
            return;
        }
        c -= a[l];
        l += 1;
    }
    println!("No");
}
