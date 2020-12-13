use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    if m == 0 {
        println!("1");
    } else {
        input! {mut a: [usize; m]}
        a.sort();
        let mut min = n;
        if a[0] > 1 {
            min = a[0] - 1;
        }
        for i in 1..m {
            if a[i] - a[i - 1] > 1 {
                min = min.min(a[i] - a[i - 1] - 1);
            }
        }
        if n - a[m - 1] > 0 {
            min = min.min(n - a[m - 1]);
        }

        if min == n {
            println!("0");
        } else {
            let mut c = 0;
            if a[0] > 1 {
                c += (a[0] - 1) / min;
                if (a[0] - 1) % min != 0 {
                    c += 1;
                }
            }
            for i in 1..m {
                if a[i] - a[i - 1] > 1 {
                    c += (a[i] - a[i - 1] - 1) / min;
                    if (a[i] - a[i - 1] - 1) % min != 0 {
                        c += 1;
                    }
                }
            }
            if n - a[m - 1] > 0 {
                c += (n - a[m - 1]) / min;
                if (n - a[m - 1]) % min != 0 {
                    c += 1;
                }
            }
            println!("{}", c);
        }
    }
}
