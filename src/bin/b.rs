fn main() {
    proconio::input! {
        s: String,
        t: String,
    }

    println!();
    println!("{}", if t.starts_with(&s) {"Yes"} else {"No"});
}
