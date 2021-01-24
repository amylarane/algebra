use std::collections::HashMap;

pub fn array_to_hash_map<T: Clone + Eq + std::hash::Hash, R: Clone>(
    array: &[(T, R)],
) -> HashMap<T, R> {
    array.iter().cloned().collect()
}

pub fn opt_to_string(string: Option<String>) -> String {
    match string {
        Some(string) => string,
        None => "".to_string(),
    }
}

pub fn pull(s: &String) -> (Option<char>, Option<String>) {
    let s = s.trim().chars().collect::<String>();
    (
        s.chars().next(),
        match s.len() {
            0 | 1 => None,
            _ => Some(get_rest(&s, 1)),
        },
    )
}

pub fn get_at(s: &String, index: usize) -> Option<char> {
    s.chars().skip(index).next()
}

pub fn get_rest(s: &String, index: usize) -> String {
    s.chars()
        .skip(index)
        .collect::<String>()
        .trim()
        .chars()
        .collect()
}
