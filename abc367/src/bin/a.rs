fn main() {
    proconio::input! {
        a: u8,
        b: u8,
        c: u8,
    }
    let f = if c < b {
        c < a && a < b
    } else {
        c < a || a < b
    };
    println!("{}", if f {"Yes"} else {"No"});
}
