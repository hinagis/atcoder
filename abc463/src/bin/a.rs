use proconio::input as I;

fn main() {
    I! {
        x: u32,
        y: u32
    }
    println!("{}", if (x / 16) * 9 == y && (y / 9) * 16 == x {"Yes"} else {"No"});
}
