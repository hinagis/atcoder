use proconio::input;

fn main() {
    input! {
        n: usize,
        d: [(i32, i32); n],
    }
    let r = n;
    for (_a, _b) in d {
        todo!();
    }

    println!("Yes");
    println!("{}", r);
}
