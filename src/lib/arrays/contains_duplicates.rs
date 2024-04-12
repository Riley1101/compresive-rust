use std::collections::HashSet;

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
