//use proconio::{input, fastout};
use proconio::input;
use std::collections::{HashSet, HashMap};

//#[fastout]
fn main() {
    input! {
        n: i32,
        m: i32,
        r: [(i32, i32); m],
    }
    let mut done: HashMap<i32, i32> = HashMap::new();

    let mut rooms: HashMap<i32, HashSet<i32>> = HashMap::new();
    for i in 1..(n + 1) {
        rooms.insert(i, HashSet::new());
    }
    for e in r {
        rooms.get_mut(&e.0).unwrap().insert(e.1);
        rooms.get_mut(&e.1).unwrap().insert(e.0);
    }

    let mut next: HashSet<i32> = HashSet::new();
    next.insert(1);

    while next.len() > 0 {
        let now = next;
        next = HashSet::new();
        for n in now {
            for e in rooms.get(&n).unwrap() {
                if *e != 1 && !done.contains_key(e) {
                    next.insert(*e);
                    done.insert(*e, n);
                }
            }
        }
    }
    let mut v: Vec<(i32, i32)> = Vec::new();
    for e in done {
        v.push(e);
    }
    v.sort_by(|(k1, _v1), (k2, _v2)| k1.cmp(k2));
    println!("Yes");
    for (_k, v) in v {
        println!("{}", v);
    }
}
