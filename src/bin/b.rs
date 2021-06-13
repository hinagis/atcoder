fn main() {
    proconio::input! {
        n: usize,
        mut a: [usize; n],
    }
    a.sort();

    let mut f = true;
    for i in 0..n {
        if a[i] != i + 1 {
            f = false;
            break;
        }
    }
    println!("{}", if f {"Yes"} else {"No"});
}
