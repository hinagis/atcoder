use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        mut a: i64,
        mut b: i64,
        k: i64,
    }

    let mut l = 0;
    while a > 0 && b > 0 {
        let c = comb(a - 1 + b, a - 1);
        if k <= l + c {
            print!("{}", 'a');
            a -= 1;
        } else {
            print!("{}", 'b');
            b -= 1;
            l += c;
        }
    }
    while a > 0 {
        print!("{}", 'a');
        a -= 1;
    }
    while b > 0 {
        print!("{}", 'b');
        b -= 1;
    }
    println!("");
}

fn comb(n: i64, r: i64) -> i64 {
    (0..n + 1)
        .rev()
        .zip(1..r + 1)
        .fold(1, |mut r, (n, d)| { r *= n; r /= d; r })
}
