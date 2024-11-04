use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(phrase: &str) -> HashMap<String, u32> {
  let lower: String = phrase.to_lowercase();
  let words: Vec<&str> = lower.split_whitespace().collect();
  let mut counts: HashMap<String, u32> = HashMap::new();

  for word in words {
    let word = String::from(word);
    if let Some(c) = counts.get(&word) {
      counts.insert(word, *c + 1);
    } else {
      counts.insert(word, 1);
    }
  }
  counts
}