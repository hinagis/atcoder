use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n]
    }
    let mut x = xy.iter().map(|&(x, _)| x).collect::<Vec<_>>();
    let mut y = xy.iter().map(|&(_, y)| y).collect::<Vec<_>>();
    x.sort();
    y.sort();

    let mut r = 0;
    let m = n / 2;
    for i in 0..n {
        r += (x[i] - x[m]).abs() + (y[i] - y[m]).abs();
    }
    println!("{}", r);
}
