fn main() {
    proconio::input! {
        s: char,
        t: char,
    }

    println!("{}", if s == 'Y' {t.to_ascii_uppercase()} else {t});
}
