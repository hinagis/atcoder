fn main() {
    proconio::input! {
        a: u64,
        b: u64,
        c: u64,
    }
    let g = gcd(gcd(a, b), c);
    println!("{}", (a + b + c) / g - 3);
}

fn gcd(a: u64, b: u64) -> u64 {
    match b {
        0 => a,
        _ => gcd(b, a % b)
    }
}
