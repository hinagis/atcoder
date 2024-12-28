fn main() {
    proconio::input! {s: String}
    let mut t = 0;
    let mut z = false;
    for c in s.chars() {
        if c == '0' {
            if z {
                t += 1;
            }
            z ^= true;
        } else {
            if z {
                t += 1;
                z = false;
            }
            t += 1;
        }
    }
    if z {
        t += 1;
    }
    println!("{}", t);
}
