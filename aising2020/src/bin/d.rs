fn main() {
    proconio::input! {
        n: usize,
        x: String,
    }

    let bp = x.match_indices('1').count() as u32;
    let p = if bp == 1 { [2, 1] } else { [bp + 1, bp - 1] };

    let xr: Vec<char> = x.chars().rev().collect();

    let mut pows = vec![[1, 1]; n];
    let mut xv = if xr[0] == '1' { [1, 1] } else { [0, 0] };
    for i in 1..n {
        for j in 0..=1 {
            pows[i][j] = (pows[i - 1][j] * 2) % p[j];
            if xr[i] == '1' {
                xv[j] = (xv[j] + pows[i][j]) % p[j];
            }
        }
    }

    for i in (0..n).rev() {
        if bp == 1 && xr[i] == '1' {
            println!("0");
        } else {
            let xi = if xr[i] == '0' {
                let j = 0;
                (xv[j] + pows[i][j]) % p[j]
            } else {
                let j = 1;
                let p = if xv[j] >= pows[i][j] { 0 } else { p[j] };
                xv[j] + p - pows[i][j]
            };
            println!("{}", 1 + pop_pop_count(xi));
        }
    }
}

fn pop_pop_count(mut v: u32) -> u32 {
    let mut r = 0;
    while v > 0 {
        v %= v.count_ones() as u32;
        r += 1;
    }
    r
}
