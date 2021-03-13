fn main() {
    proconio::input! {
        a: usize,
        b: usize,
        w: usize,
    }
    let w = w * 1000;
    let wa = w / a;
    let wb = w / b;

    if a * wa != w && b * wb != w && wa == wb {
        println!("UNSATISFIABLE");
    } else {
        println!("{} {}", if wb * b == w {wb} else {wb + 1}, wa);
    }
}
