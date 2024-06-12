fn shortest_word(s: &str) -> Option<&str> {
    let words: Vec<&str> = s.split_whitespace().collect();
    words.iter().min_by_key(|s| s.len()).cloned().next()
}