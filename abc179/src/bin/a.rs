fn main() {
    proconio::input! {
        s: String,
    }

    println!("{}", if (&s).ends_with("s") { s + "es" } else { s + "s" });
}
