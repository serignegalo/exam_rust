use std::collections::HashMap;
pub fn counting_words(words: &str) -> HashMap<String, u32> {
    let mut res: HashMap<String, u32> = HashMap::new();
    let mut sentence = words.to_ascii_lowercase();
    let mut punctuations = vec!['“', '”', '―'];
    //on peuple le vec de tous les ponctuations sauf '
    for c in sentence.chars() {
        if c.is_ascii_punctuation() && c != '\'' {
            punctuations.push(c);
        }
    }
    // on enlève les ponctuations sauf '
    for c in punctuations {
        if sentence.contains(c) {
            sentence = sentence.replace(c, "");
        }
    }
    
    for word in sentence.split_whitespace() {
        let clean_word = word.trim_matches('\''); // pour gerer le test avec 'large'
        *res.entry(clean_word.to_string()).or_insert(0) += 1;
    }
    res
}
