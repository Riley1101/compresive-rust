use std::collections::{HashMap, HashSet};

pub fn contains_dups() -> bool {
    let v = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1];
    let mut s: HashSet<i32> = HashSet::new();

    for i in v {
        if s.contains(&i) {
            return true;
        }
        s.insert(i);
    }
    false
}

pub fn valid_anagram(s: String, t: String) -> bool {
    let mut set: HashMap<char, usize> = HashMap::new();

    for i in s.chars() {
        let x = set.get_mut(&i);
        match x {
            Some(val) => *val += 1,
            None => {
                set.insert(i, 1);
            }
        }
    }
    for j in t.chars() {
        let x = set.get_mut(&j);
        match x {
            Some(val) => {
                *val -= 1;
            }
            None => {
                return false;
            }
        }
    }
    true
}
