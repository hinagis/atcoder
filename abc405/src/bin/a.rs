fn main() {
    proconio::input! {
        r: u32,
        x: u8
    }
    println!("{}", if x == 1 {if r >= 1600 && r <= 2999 {"Yes"} else {"No"}} else {if r >= 1200 && r <= 2399 {"Yes"} else {"No"}});
}
