fn main() {
    proconio::input! {
        r: usize,
        c: usize,
    }

    let (l, r) = (r, 16 - r);
    let (l, r) = if l <= r {(l, r)} else {(r, l)};
    println!("{}", if c >= l && c <= r {
            if r % 2 == 0 {"white"} else {"black"}
        } else {
            if c % 2 == 0 {"white"} else {"black"}
        }
    );
}
