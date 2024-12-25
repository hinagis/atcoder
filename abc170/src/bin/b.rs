use proconio::input;

fn main() {
    input! {
        x: i32,
        y: i32,
    }
    for t in 0..(x + 1) {
        if 2 * t == y - 2 * x {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
