fn main() {
    proconio::input! {n: u64}
    let mut k = 1;
    for x in 1..n {
        let j = x.pow(3);
        if j > n {break}

        let mut f = true;
        let s = j.to_string().chars().collect::<Vec<_>>();
        let m = s.len();
        for i in 0..m / 2 {
            if s[i] != s[m - 1 - i] {
                f = false;
                break;
            }
        }
        if f {
            k = j;
        }
    }
    println!("{}", k);
}
