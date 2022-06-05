fn main() {
    proconio::input! {n: usize}

    let mut d = (0..=n).collect::<Vec<usize>>();
    let mut i = 2;
    while i * i <= n {
        if d[i] == i {
            let mut j = i * i;
            while j <= n {
                while d[j] % (i * i) == 0 {
                    d[j] /= i * i;
                }
                j += i * i;
            }
        }
        i += 1;
    }

    let mut h = vec![0; n + 1];
    let mut i = 1;
    while i * i <= n {
        h[i * i] += 1;
        i += 1;
    }
    for i in 1..=n {
        h[i] += h[i - 1];
    }

    let mut r = 0;
    for i in 1..=n {
        r += h[n / d[i]];
    }
    println!("{}", r);
}
