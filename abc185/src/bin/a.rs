fn main() {
    proconio::input! {
        a: [u32; 4],
    }
    let mut r = a[0];
    for i in 1..4 {
        r = r.min(a[i]);
    }

    println!("{}", r);
}
