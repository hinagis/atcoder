fn main() {
    proconio::input! {
        a: u64,
        n: u64,
    }
    let mut r = 0;
    let mut i = 1;
    let mut k = 1;
    while i <= n {
        if calc(i, a) {
            r += i;
        }
        let m = 10u64.pow(k / 2);
        i /= m;
        i += 1;
        if i * m >= 10u64.pow(k) {
            if k % 2 == 1 {
                i /= 10;
            }
            k += 1;
        }
        let mut j = if k % 2 == 0 {i} else {i / 10};
        while j > 0 {
            i *= 10;
            i += j % 10;
            j /= 10;
        }
    }

    println!("{}", r);
}

fn calc(mut n: u64, a: u64) -> bool {
    let mut b = vec![];
    while n > 0 {
        b.push(n % a);
        n /= a;
    }
    let n = b.len();
    for i in 0..n / 2 {
        if b[i] != b[n - i - 1] {
            return false;
        }
    }
    true
}
