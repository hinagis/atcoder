fn main() {
    proconio::input! {
        n: usize,
        xy: [(i32, i32); n]
    }
    let mut r = 0;

    for i in 0..n {
        let (x0, y0) = xy[i];
        for j in (i + 1)..n {
            let (x1, y1) = xy[j];
            if (y1 - y0).abs() <= (x1 -x0).abs() {
                r += 1;
            }
        }
    }
    println!("{}", r);
}
