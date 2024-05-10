use compresive_rust::arrays::contains_duplicates::valid_anagram;

fn main() {
    let val = valid_anagram("rat".to_string(), "car".to_string());
    assert_eq!(val, false);
}
