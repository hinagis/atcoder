fn main() {
    proconio::input! {
        mut x: i128,
        mut k: i128,
        d: i128,
    }

    x = x.abs();
    if x > k * d {
        x -= k * d;
    } else {
        let xd = x / d;
        k -= xd;
        x -= xd * d;
        if k % 2 == 1 {
            x -= d;
        }
    }

    println!("{}", x.abs());
}
