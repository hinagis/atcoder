use proconio::input;

fn main() {
    input! {t: usize}

    for _ in 0..t {
        input! {
            a: u64,
            s: u64,
        }
        if 2 *a > s {
            println!("No");
        } else {
            let s = s - 2 * a;
            if ! a & s == s {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    }
}
