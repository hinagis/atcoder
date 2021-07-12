fn main() {
    proconio::input! {
        h: u32,
        w: u32
    }
    println!("{}", if h == 1 || w == 1 {h * w} else {((h + 1) / 2) * ((w + 1) / 2)});
}
