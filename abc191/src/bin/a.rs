fn main() {
    proconio::input! {
        v: u32,
        t: u32,
        s: u32,
        d: u32,
    }
    let (t, s) = (v * t, v * s);

    println!("{}", if d < t || d > s {"Yes"} else {"No"});
}
