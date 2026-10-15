use std::collections::HashMap;

pub fn word_frequencies(input: &str) -> std::collections::HashMap<String, usize> {
    let mut result: HashMap<String, usize> = HashMap::new();

    for word in input.split_whitespace() {
        match result.get_mut(word) {
            Some(val) => {
                *val += 1;
            }
            None => {
                result.insert(word.to_string(), 1);
            }
        };
    }
    result
}
