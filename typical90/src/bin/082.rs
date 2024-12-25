const M: u128 = 1000_000_007;

fn main() {
    proconio::input! {
        l: u128,
        r: u128,
    }
    let mut y = 0;
    let mut c = 1;
    let mut s = 1;
    while s <= r {
        let x = if l < s * 10 {
            let l = if s <= l {l} else {s};
            let r = if r < s * 10 {r} else {s * 10 - 1};
            ((r + 1 - l) * (l + r) / 2) % M
        } else {
            0
        };
        y = (y + (x * c % M)) % M;

        c += 1;
        s *= 10;
    }
    println!("{}", y);
}
