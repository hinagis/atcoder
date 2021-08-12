fn main() {
    proconio::input! {
        h: usize,
        w: usize,
        a: [[i64; w]; h],
        b: [[i64; w]; h],
    }

    let mut s = 0;
    let mut c = vec![vec![0; w]; h];
    for y in 0..h {
        for x in 0..w {
            c[y][x] = b[y][x] - a[y][x];
            s += c[y][x];
        }
    }
    if s % 4 == 0 {
        println!("Yes");

        let mut s = 0;
        for y in 0..h - 1 {
            for x in 0..w - 1 {
                s += c[y][x].abs();
                c[y + 1][x + 1] -= c[y][x];
                c[y + 1][x] -= c[y][x];
                c[y][x + 1] -= c[y][x];
                c[y][x] -= c[y][x];
            }
        }
        println!("{}", s);
    } else {
        println!("No");
    }
}
