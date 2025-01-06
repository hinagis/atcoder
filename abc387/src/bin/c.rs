fn main() {
    proconio::input! {l: u64, r: u64}
    println!("{}", calc(r) - calc(l - 1));
}

fn calc(n: u64) -> u64 {
    let s: Vec<_> = n.to_string().chars().map(|e| (e as u8 - b'0') as u64).collect();
    let n = s.len();
    let mut r = if s.iter().skip(1).all(|&c| c < s[0]) {1} else {0};
    for i in 1..n {
        r += s[0].pow((n - i - 1) as u32) * s[0].min(s[i]);
        if s[i] >= s[0] {
            break;
        }
    }
    r += (1..s[0]).map(|c| c.pow(n as u32 - 1)).sum::<u64>();
    r += (1..n).map(|i| (1..=9u64).map(|c| c.pow(i as u32 - 1)).sum::<u64>()).sum::<u64>();
    r
}