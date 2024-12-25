fn rl() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s
}

fn check(s: &str) -> bool {
    let mut sts = "de";
    let mut it = s.chars();

    while sts != "ok" && sts != "ng" {
        sts = match sts {
            "de" => {
                match it.next() {
                    Some('d') => "ream",
                    Some('e') => "rase",
                    None => "ok",
                    _ => "ng",
                }
            },
            "ream" => {
                let result = it.next() == Some('r');
                let result = result && it.next() == Some('e');
                let result = result && it.next() == Some('a');
                let result = result && it.next() == Some('m');
                if result {
                    match it.next() {
                        Some('d') => "ream",
                        Some('e') => "r",
                        None => "ok",
                        _ => "ng",
                    }
                } else {
                    "ng"
                }
            },
            "rase" => {
                let result = it.next() == Some('r');
                let result = result && it.next() == Some('a');
                let result = result && it.next() == Some('s');
                let result = result && it.next() == Some('e');
                if result {
                    match it.next() {
                        Some('d') => "ream",
                        Some('e') => "rase",
                        Some('r') => "de",
                        None => "ok",
                        _ => "ng",
                    }
                } else {
                    "ng"
                }
            },
            "r" => {
                let result = it.next() == Some('r');
                if result {
                    match it.next() {
                        Some('a') => "se",
                        Some('d') => "ream",
                        Some('e') => "rase",
                        None => "ok",
                        _ => "ng",
                    }
                } else {
                    "ng"
                }
            },
            "se" => {
                let result = it.next() == Some('s');
                let result = result && it.next() == Some('e');
                if result {
                    match it.next() {
                        Some('d') => "ream",
                        Some('e') => "rase",
                        Some('r') => "de",
                        None => "ok",
                        _ => "ng",
                    }
                } else {
                    "ng"
                }
            },
            _ => "ng",
        }
    }
    sts == "ok"
}

fn main() {
//    let r = regex::Regex::new("^(dream(er)?|erase(r)?)+$").unwrap();
    let s = rl();
    let s = s.trim();
//    if r.is_match(s) {
    if check(s) {
        println!("YES");
    } else {
        println!("NO");
    }
}
