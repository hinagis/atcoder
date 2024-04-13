use proconio::{input as I, fastout as F};

#[F]
fn main() {
    I! {
        l: u64,
        r: u64,
    }
    fn get_i(mut l: u64) -> u32 {
        if l == 0 {
            return 60;
        }
        let mut i = 0;
        while l % 2 == 0 {
            i += 1;
            l /= 2;
        }
        i as u32
    }
    fn calc(q: &mut Vec<u64>, l: u64, r: u64) {
        let i = get_i(l);
        for i in (0..=i).rev() {
            let a = 2u64.pow(i);
            let j = l / a;
            let l = a * (j + 1);
            if l <= r {
                q.push(l);
                calc(q, l, r);
                break;
            }
        }
    }
    let mut a = vec![l];
    calc(&mut a, l, r);
    println!("{}\n{}",
        a.len() - 1,
        a.windows(2)
            .map(|e| e.iter()
                .map(|e| e.to_string())
                .collect::<Vec<_>>()
                .join(" ")
            )
            .collect::<Vec<_>>()
            .join("\n")
    );
}
