fn main() {
    proconio::input! {
        s: String,
        t: String
    }
    println!("{}", if s == "AtCoder" && t == "Land" {"Yes"} else {"No"});
}
