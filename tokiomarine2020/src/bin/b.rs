use proconio::input;

fn main() {
    input! {
        a: i128,
        v: i128,
        mut b: i128,
        w: i128,
        t: i128,
    }
    b -= a;
    b = b.abs();

    if b - (v - w) * t <= 0 {
        println!("YES");
    } else {
        println!("NO");
    }
}
