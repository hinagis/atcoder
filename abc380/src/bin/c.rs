fn main() {
    proconio::input! {
        n: usize,
        k: usize,
        s: String,
    }
    let mut p = '0';
    let mut i = 0;
    let mut z = 0;
    let mut t = Vec::with_capacity(n);
    for c in s.chars() {
        if c == '1' && c != p {
            i += 1;
        }
        if c == '0' && i + 1 == k {
            z += 1;
        } else {
            t.push(c);
        }
        if c == '0' && i == k {
            while z > 0 {
                t.push('0');
                z -= 1;
            }
        }
        p = c;
    }
    while z > 0 {
        t.push('0');
        z -= 1;
    }
    println!("{}", t.iter().collect::<String>());
}
