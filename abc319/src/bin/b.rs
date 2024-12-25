fn main() {
    proconio::input! {n: usize}
    let mut g = vec![];
    for i in 1..=9 {
        if n % i == 0 {
            g.push(i);
        }
    }
    let mut r = String::new();
    for i in 0..=n {
        let mut f = true;
        for &j in &g {
            if i % (n / j) == 0 {
                f = false;
                r.push((j as u8 + b'0') as char);
                break;
            }
        }
        if f {
            r.push('-');
        }
    }
    println!("{}", r);
}
