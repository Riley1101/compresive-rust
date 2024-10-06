#![allow(dead_code)]
use std::collections::{HashMap, HashSet};

pub fn contain_duplicates(vec: Vec<i32>) -> bool {
    let mut set = HashSet::new();
    for i in vec.iter() {
        if set.contains(&i) {
            return true;
        }
        set.insert(i);
    }
    false
}

#[test]
fn is_dups() {
    let vec = [1, 2, 3, 4, 5];
    assert!(!contain_duplicates(vec.to_vec()));
}

fn valid_anagram(t: String, s: String) -> bool {
    if s.len() != t.len() {
        return false;
    }
    let mut map = HashMap::new();
    for i in t.chars() {
        map.entry(i)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }
    for i in s.chars() {
        let val = map.get_mut(&i);
        if let Some(val) = val {
            if val == &0 {
                return false;
            } else {
                *val -= 1;
            }
        } else {
            return false;
        };
    }
    true
}

fn valid_anagram_2(s: String, t: String) -> bool {
    let mut map = HashMap::new();
    s.chars().for_each(|c| *map.entry(c).or_insert(0) += 1);
    t.chars().for_each(|c| *map.entry(c).or_insert(0) -= 1);
    map.into_values().any(|c| c != 0)
}

#[test]
fn is_anagram() {
    assert!(valid_anagram("ta".to_string(), "at".to_string()));
    assert!(valid_anagram_2("ta".to_string(), "at".to_string()));
}
