fn main() {
    proconio::input! {n: u32}
    println!("{}", if n == 191 {"so-so"} else if n < 191 {"Yay!"} else {":("});
}
