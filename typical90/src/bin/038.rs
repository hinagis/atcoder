fn main() {
    proconio::input! {a: u128, b: u128}
    let c = a * b / gcd(a, b);
    const M: u128 = 1000_000_000_000_000_000;
    println!("{}", if c > M {"Large".to_string()} else {c.to_string()});
}

fn gcd(a: u128, b: u128) -> u128 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
