fn main() {
    proconio::input! {
        n: usize,
        p: i64,
        k: usize,
        a: [[i64; n]; n]
    }

    let calc = |x: i64| {
        let mut a = a.clone().iter().map(
            |a| a.iter().map(|&a| if a == -1 {x} else {a}).collect::<Vec<_>>()
        ).collect::<Vec<_>>();
        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    a[i][j] = a[i][j].min(a[i][k] + a[k][j]);
                }
            }
        }
        (a.iter().flatten().filter(|&&a| a <= p).count() - n) / 2
    };

    if calc(p + 1) == k {
        println!("Infinity");
        return;
    }

    let f = |k| {
        let mut l = 0;
        let mut r = p + 1;
        while r - l > 1 {
            let x = (l + r) / 2;
            if calc(x) <= k {
                r = x;
            } else {
                l = x;
            }
        }
        r
    };

    println!("{}", f(k - 1) - f(k));
}
