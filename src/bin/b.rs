fn main() {
    proconio::input! {
        a: i64,
        b: i64,
        c: i64,
        d: i64,
    }

    let ac = a * c;
    let ad = a * d;
    let bc = b * c;
    let bd = b * d;
    println!("{}", ac.max(ad.max(bc.max(bd))));
}
