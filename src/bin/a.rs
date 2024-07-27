use proconio::input as I;

fn main() {
    I! {n: usize}
    let mut c = 0;
    for _ in 0..n {
        if c >= 2 {
            println! {"No"};
            return;
        }

        I! {s: String}
        if s == "sweet" {
            c += 1;
        } else {
            c = 0;
        }
    }
    println!("Yes");
}
