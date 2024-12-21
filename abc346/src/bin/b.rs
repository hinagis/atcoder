fn main() {
    proconio::input! {
        w: i32,
        b: i32
    }

    let k = [(9, 4), (7, 3), (6, 4), (5, 4), (3, 3)];
    let m = (b % 5) as usize;
    let (l, r) = if b == 0 {
        (1, 2)
    } else {
        let l = 7 * ((b / 5) + 1) - k[m].0;
        (l, l + k[m].1)
    };
    println!("{}", if w >= l && w <= r {"Yes"} else {"No"});
}
