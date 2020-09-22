use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut wh = vec![(n, n - 2)];
    let mut hw = vec![(n, n - 2)];
    let mut r = (n - 2) * (n - 2);
    for _ in 0..q {
        input! {f: usize, x: usize}
        let (wh, hw) = if f == 1 {(&mut wh, &mut hw)} else {(&mut hw, &mut wh)};
        if x < wh.last().unwrap().0 {
            let h = hw.last().unwrap().0;
            r -= h - 2;
            wh.push((x, h - 2));
        } else {
            for i in 0..wh.len() {
                if x > wh[i].0 {
                    r -= wh[i].1;
                    break;
                }
            }
        }
    }
    println!("{}", r);
}
