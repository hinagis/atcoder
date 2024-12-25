fn main() {
    proconio::input! {
        a: usize,
        b: usize,
        c: usize,
    }

    const T: &str = &"Takahashi";
    const A: &str = &"Aoki";
    println!("{}", if c == 0 {
        if a > b {T} else {A}
    } else {
        if b > a {A} else {T}
    });
}
