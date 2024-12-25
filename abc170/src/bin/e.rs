use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        ab: [(usize, usize); n],
        _cd: [(usize, usize); q],
    }
    let r = n;
    for (_a, _b) in ab {
        todo!();
    }

    println!("Yes");
    println!("{}", r);
}
