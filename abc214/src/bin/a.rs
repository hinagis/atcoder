fn main() {
    proconio::input! {
        n: usize,
    }

    println!("{}", match n {
        1..=125 => 4,
        126..=211 => 6,
        _ => 8
    });
}
