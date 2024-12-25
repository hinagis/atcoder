fn main() {
    proconio::input! {
        n: u32,
        m: usize,
        t: u32,
        ab: [(u32, u32); m]
    }

    let mut nt = 0;
    let mut r = n;
    for (a, b) in ab {
        let a = a - nt;
        if r <= a {
            println!("No");
            return;
        }
        r -= a;
        nt += a;
        let b = b - nt;
        r = if r + b > n {n} else {r + b};
        nt += b;
    }
    println!("{}", if r > t - nt {"Yes"} else {"No"});
}
