fn main() {
    proconio::input! {
        h: usize,
        w: usize,
        c: i64,
        a: [[i64; w]; h]
    }
    let mut b = vec![(0, 0, 0); h * w];
    for i in 0..h {
        for j in 0..w {
            b[i * w + j] = (a[i][j], i, j);
        }
    }
    b.sort();
    let mut r = b[0].0 + b[1].0 + c * ((b[0].1 as i64 - b[1].1 as i64).abs() + (b[0].2 as i64 - b[1].2 as i64).abs());
    for i in 0..10000.min(h * w) {
        for j in i + 1..100000.min(h * w) {
            r = r.min(b[i].0 + b[j].0 + c * ((b[i].1 as i64 - b[j].1 as i64).abs() + (b[i].2 as i64 - b[j].2 as i64).abs()));
        }
    }
    /*
    let mut r = a[0][0] + a[0][1] + c;
    for i in 0..h {
        for j in 0..w {
            for x in j + 1..w {
                r = r.min(a[i][j] + a[i][x] + c * ((x - j) as u64));
            }
            for y in i + 1..h {
                r = r.min(a[i][j] + a[y][j] + c * ((y - i) as u64));
                for x in 0..j {
                    r = r.min(a[i][j] + a[y][x] + c * (((y - i) + (j - x)) as u64));
                }
            }
            for y in i + 1..h {
                for x in j + 1..w {
                    r = r.min(a[i][j] + a[y][x] + c * (((y - i) + (x - j)) as u64));
                }
            }
        }
    }
    */

    println!("{}", r);
}
