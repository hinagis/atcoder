use proconio::{input as I, fastout as F};

#[F]
fn main() {
    I! {
        n: usize,
        a: [i32; n],
        q: usize,
    }
    let mut t = std::collections::HashMap::new();
    t.insert(a[0], (None, Some(a[1])));
    for i in 1..n - 1 {
        t.insert(a[i], (Some(a[i - 1]), Some(a[i + 1])));
    }
    t.insert(a[n - 1], (Some(a[n - 2]), None));
    for _ in 0..q {
        I! {
            k: u8,
            x: i32
        }
        if k == 1 {
            I! {y: i32}
            let (_, v) = t.get(&x).unwrap();
            t.insert(y, (Some(x), *v));
            (*t.get_mut(&x).unwrap()).1 = Some(y);
        } else {
            let (u, v) = t.get(&x).unwrap();
            if *u == None && *v == None {
            } else if *u == None {
                let v = ((*v).unwrap()).clone();
                (*t.get_mut(&v).unwrap()).0 = None;
            } else if *v == None {
                let u = ((*u).unwrap()).clone();
                (*t.get_mut(&u).unwrap()).1 = None;
            } else {
                let u = ((*u).unwrap()).clone();
                let v = ((*v).unwrap()).clone();
                (*t.get_mut(&u).unwrap()).1 = Some(v);
                (*t.get_mut(&v).unwrap()).0 = Some(u);
            }
            t.remove(&x);
        }
    }
    fn get_first(t: &std::collections::HashMap<i32, (Option<i32>, Option<i32>)>) -> i32 {
        for (k, v) in t {
            if v.0 == None {
                return *k;
            }
        }
        return 0
    }
    let mut r = vec![];
    let mut x = get_first(&t);
    r.push(x);
    loop {
        let (_, v) = t.get(&x).unwrap();
        if let Some(v) = v {
            x = *v;
            r.push(x);
        } else {
            break;
        }
    }
    println!("{}", r.iter().map(|e| e.to_string()).collect::<Vec<_>>().join(" "));
}
