use proconio::input as I;

fn main() {
    I! {x: u32}
    println!("{}", if x > 2 && x < 19 {"Yes"} else {"No"} );
}
