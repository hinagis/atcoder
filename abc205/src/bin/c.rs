fn main() {
    proconio::input! {
        mut a: i64,
        mut b: i64,
        mut c: i64,
    }

    c %= 2;
    if c == 0 {
        a *= a;
        b *= b;
    }
    println!("{}", if a == b {"="} else if a > b {">"} else {"<"});
}
