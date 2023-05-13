fn main() {
    proconio::input! {
        n: usize,
        s: String
    }
    let h: usize = n / 2 + if n % 2 == 0 {0} else {1};
    let mut t = 0;
    let mut a = 0;
    for c in s.chars() {
        if c == 'T' {
            t += 1;
            if t >= h {
                println!("T");
                return;
            }
        } else {
            a += 1;
            if a >= h {
                println!("A");
                return;
            }
        }
    }
}
