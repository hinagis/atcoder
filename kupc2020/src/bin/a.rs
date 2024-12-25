use proconio::input;

fn main() {
    input! {
        n: usize,
        mut x: i32,
        mut y: i32,
    }

    let mut s = 0;
    for _ in 1..n {
        input! {j: i32, i: i32}
        s += (x - j).abs() + (y - i).abs();
        x = j;
        y = i;
    }
    println!("{}", s);
}
