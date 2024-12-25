fn main() {
    proconio::input! {
        n: usize,
        y: usize,
    }

    let mut r = vec![n, 0, 0];

    while r[0] * 10000 + r[1] * 5000 + r[2] * 1000 != y {
        if r[1] == 0 {
            if r[0] == 0 {
                println!("-1 -1 -1");
                return;
            } else {
                r[0] -= 1;
                r[1] += r[2] + 1;
                r[2] = 0;
            }
        } else {
            r[1] -= 1;
            r[2] += 1;
        }
    }
    println!("{}", r.iter().map(|r| r.to_string()).collect::<Vec<_>>().join(" "));
}
