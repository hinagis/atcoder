fn main() {
    proconio::input! {
        a: usize,
        b: usize,
        c: usize,
    }

    println!("{}", if a*a + b*b < c*c {"Yes"} else {"No"});
}
