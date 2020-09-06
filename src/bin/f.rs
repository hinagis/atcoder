use {
    proconio::{input, marker::Usize1},
    std::{
        collections::{BTreeSet, BTreeMap},
        ops::Bound::Included,
    },
};

const M: usize = usize::max_value();

fn main() {
    input! {
        h: usize,
        w: usize,
        ab: [(Usize1, Usize1); h],
    }

    let mut es = BTreeSet::new();
    let mut dc = BTreeMap::new();
    for j in 0..w {
        es.insert((j, j));
    }
    dc.insert(0, w);

    for i in 0..h {
        let (a, b) = ab[i];
        let rng = es.range((Included(&(a, 0)), Included(&(b, w))));
        let add = if let Some(&(_, s)) = rng.clone().last() {
            let e = if b == w - 1 { M } else { b + 1 };
            Some((e, s))
        } else {
            None
        };

        let mut rm = vec![];
        for &(e, s) in rng {
            rm.push((e, s));
        }

        for (e, s) in rm {
            es.remove(&(e, s));

            let d = e - s;
            if let Some(v) = dc.get_mut(&d) {
                if *v <= 1 {
                    dc.remove(&d);
                } else {
                    *v -= 1;
                }
            }
        }

        if let Some((e, s)) = add {
            es.insert((e, s));
            *dc.entry(e - s).or_insert(0) += 1;
        }

        let d = if let Some(f) = dc.iter().next() { *f.0 } else { M };
        if d >= w {
            println!("-1");
        } else {
            println!("{}", d + i + 1);
        }
    }
}
