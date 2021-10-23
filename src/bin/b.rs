fn main() {
    proconio::input! {
        h: usize,
        w: usize,
        a: [[u32; w]; h]
    }

    for i in 0..h {
        for j in 0..w {
            for y in i + 1..h {
                for x in j + 1..w {
                    if a[i][j] + a[y][x] > a[y][j] + a[i][x] {
                        println!("No");
                        return;
                    }
                }
            }
        }
    }
    println!("Yes");
}
