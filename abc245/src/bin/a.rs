fn main() {
    proconio::input! {
        a: u32,
        b: u32,
        c: u32,
        d: u32,
    }

    println!("{}", if a * 60 + b <= c * 60 + d {"Takahashi"} else {"Aoki"});
}
