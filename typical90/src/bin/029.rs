use proconio::{input as I, fastout as F};

#[F]
fn main() {
    I! {
        w: usize,
        q: [(usize, usize)]
    }

    let mut t = std::collections::BTreeMap::new();
    t.insert(0, 0);
    t.insert(w, 0);

    for &(l, r) in &q {
        let (_, &hl) = t.range(..l).last().unwrap();
        let (_, &hr) = t.range(..r + 1).last().unwrap();

        let mut hl = hl + 1;
        while let Some((&i, &h)) = t.range(..r).last() {
            if i < l {
                break;
            }
            hl = hl.max(h + 1);
            t.remove(&i);
        }

        t.insert(l - 1, hl);
        t.insert(r, hr);

        println!("{}", hl);
    }
}
