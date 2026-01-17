use proconio::input as I;

fn main() {
    I! {
        p: u32,
        q: u32,
        x: u32,
        y: u32,
    }
    println!("{}", if x >= p && y >= q && x < p + 100 && y < q + 100 {"Yes"} else {"No"});
}
