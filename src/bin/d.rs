fn main() {
    proconio::input! {
        s: String,
        n: usize
    }

    let m = s.len();
    let mut r = 0;
    for (i, c) in s.bytes().enumerate() {
        if c == b'1' {
            r |= 1 << m - 1 - i;
        }
    }
    if r > n {
        println!("-1");
        return;
    }
    for (i, c) in s.chars().enumerate() {
        if c == '?' {
            let b = r | (1 << m - 1 - i);
            if b <= n {
                r = b;
            }
        }
    }
    println!("{}", r);
}
