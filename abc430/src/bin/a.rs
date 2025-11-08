use proconio::input as I;

fn main() {
    I! {
        a: u32,
        b: u32,
        c: u32,
        d: u32,
    }
    println!("{}", if c >= a && d < b {"Yes"} else {"No"});
}
