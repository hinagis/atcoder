fn main() {
    proconio::input! {
        a: u32,
        mut b: u32,
        mut c: u32,
        k: u32,
    }

    for _ in 0..k {
        if b <= a {
            b *= 2;
        } else {
            c *= 2;
        }
    }
    println!("{}", if c > b && b > a {"Yes"} else {"No"})
}
