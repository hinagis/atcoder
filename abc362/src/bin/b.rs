fn main() {
    proconio::input! {p: [(i32, i32); 3]}
    let d = [
        (p[1].0 - p[0].0).pow(2) + (p[1].1 - p[0].1).pow(2),
        (p[2].0 - p[1].0).pow(2) + (p[2].1 - p[1].1).pow(2),
        (p[0].0 - p[2].0).pow(2) + (p[0].1 - p[2].1).pow(2)
    ];
    println!("{}", if d[0] + d[1] == d[2] || d[1] + d[2] == d[0] || d[2] + d[0] == d[1] {"Yes"} else {"No"});
}
