use proconio::input as I;

fn main() {
    I! {
        n: usize,
        l: [i32; n]
    }

    println!("{}", (1..n).map(|i| (l[..i].iter().sum::<i32>() - l[i..n].iter().sum::<i32>()).abs()).min().unwrap());
}
