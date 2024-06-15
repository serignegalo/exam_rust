use std::collections::HashMap;

pub fn counting_words(words: &str) -> HashMap<String, u32> {
    let mut words_repeat: HashMap<String, u32> = HashMap::new();
    
    for word in words.split_whitespace() {
        let mut filtered_word: String = word.chars()
            .filter(|c| c.is_alphanumeric() || *c == '\'')
            .map(|v| v.to_ascii_lowercase())
            .collect();                      
        if filtered_word.starts_with("'") {
            filtered_word = filtered_word.chars().filter(|c| *c != '\'').collect();
        }
        if !filtered_word.is_empty() {
            *words_repeat.entry(filtered_word).or_insert(0) += 1;
        }
    }
    words_repeat
}
