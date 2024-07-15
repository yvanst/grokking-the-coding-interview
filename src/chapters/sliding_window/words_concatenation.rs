use std::collections::HashMap;
/// we will use a sliding window approach. this approach involves creating a window
/// of the size equal to the total length of all words combined and sliding it across
/// the string, one character at a time. for each position of the window, we will check
/// if it contains a valid concatenation of the given words.
///
/// by using a hashmap to store the frequency of each word in the list and another
/// hashmap to track the words seen in the current window, we can efficiently validate
/// the concatenations. -> move a character would only invalidate the leftmost word
struct Solution;

impl Solution {
    fn words_concatenation(text: String, words: Vec<&str>) -> Vec<usize> {
        let mut res = Vec::new();
        let mut frequency_map = HashMap::new();

        for &word in &words {
            frequency_map
                .entry(word)
                .and_modify(|c| *c += 1)
                .or_insert(1);
        }

        let word_count = words.len();
        let word_length = words[0].len();

        for i in 0..=(text.len() - word_count * word_length) {
            let mut current_map = frequency_map.clone();
            for j in 0..word_count {
                let next_word_index = i + j * word_length;
                let word = &text[next_word_index..next_word_index + word_length];
                if !frequency_map.contains_key(word) {
                    break;
                }
                current_map.entry(word).and_modify(|c| *c -= 1);
            }
            if current_map.values().all(|c| *c == 0) {
                res.push(i);
            }
        }

        res
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::words_concatenation("catfoxcat".to_string(), vec!["cat", "fox"]),
        vec![0, 3]
    );
    assert_eq!(
        Solution::words_concatenation("catcatfoxfox".to_string(), vec!["cat", "fox"]),
        vec![3]
    );
}
