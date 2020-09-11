use proconio::input;

fn main() {
    input! { t: usize }

    let mut ans = vec![];
    for _ in 0..t {
        input! { n: u64, m: u64, a: u64, b: u64 }
        ans.push(floor_sum(n, m, a, b).to_string());
    }
    println!("{}", ans.join("\n"));
}

fn floor_sum(n: u64, m: u64, mut a: u64, mut b: u64) -> u64 {
    let mut r = 0;
    if a >= m {
        r += (n - 1) * n * (a / m) / 2;
        a %= m;
    }
    if b >= m {
        r += n * (b / m);
        b %= m;
    }

    let y = (a * n + b) / m;
    if y > 0 {
        let x = y * m - b;
        r += (n - (x + a - 1) / a) * y;
        r += floor_sum(y, a, m, (a - x % a) % a);
    }
    r
}
