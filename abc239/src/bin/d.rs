fn main() {
    proconio::input! {
        a: u32,
        b: u32,
        c: u32,
        d: u32,
    }
    let p = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193, 197, 199];
    for i in a..=b {
        let mut f = true;
        for j in c..=d {
            if p.contains(&(i + j)) {
                f = false;
                break;
            }
        }
        if f {
            println!("Takahashi");
            return;
        }
    }
    println!("Aoki");
}
