fn main() {
    proconio::input! {n: u64}

    let mut c = 0;
    let mut r = n;
    let mut i = 2;
    while i * i <= n {
        while r % i == 0 {
            c += 1;
            r /= i;
        }
        i += 1;
    }

    if c == 0 {
        println!("0");
    } else {
        if r != 1 {
            c += 1;
        }
        let mut r = 1;
        let mut p = 2;
        while p < c {
            p *= 2;
            r += 1;
        }
        println!("{}", r);
    }
}
