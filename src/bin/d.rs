fn main() {
    proconio::input! {
        n: usize,
        k: usize,
        a: [[i32; n]; n],
    }

    let lim = ((k * k) / 2) + 1;
    let mut s = vec![vec![0; n + 1]; n + 1];
    let mut ng = -1;
    let mut ok = 1_000_000_000;
    while ng + 1 < ok {
        let mid = (ng + ok) / 2;
        for i in 0..n {
            for j in 0..n {
                s[i + 1][j + 1] = s[i + 1][j] + s[i][j + 1] - s[i][j];
                if a[i][j] > mid {
                    s[i + 1][j + 1] += 1;
                }
            }
        }

        if chk(&s, n, k, lim) {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{}", ok);
}

fn chk(s: &Vec<Vec<usize>>, n: usize, k: usize, lim: usize) -> bool {
    for i in 0..(n - k + 1) {
        for j in 0..(n - k + 1) {
            if s[i + k][j + k] + s[i][j] - s[i][j + k] - s[i + k][j] < lim {
                return true;
            }
        }
    }
    false
}
