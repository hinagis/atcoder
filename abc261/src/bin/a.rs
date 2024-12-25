fn main() {
    proconio::input! {
        l1: i32, r1: i32,
        l2: i32, r2: i32,
    }

    println!("{}", if r2 <= l1 || r1 <= l2 {
        0
    } else {
        let l = if l1 <= l2 {l2} else {l1};
        let r = if r1 <= r2 {r1} else {r2};
        r - l
    });
}
