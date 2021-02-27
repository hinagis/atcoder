fn main() {
    proconio::input! {
        n: usize,
        apx: [(u32, u32, u32); n]
    }

    let mut r = None;
    for &(a, p, x) in &apx {
        if x > a {
            r = Some(if let Some(r) = r {p.min(r)} else {p});
        }
    }

    if let Some(r) = r {
        println!("{}", r);
    } else {
        println!("-1");
    }
}
