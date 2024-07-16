/// this problem follows the subsets pattern and can be mapped to balanced parentheses.
/// we can follow a similar BFS approach
///
/// following a BFS approach, we will abbreviate one character at a time. at each step,
/// we have two options:
///   1. abbreviate the current character, or
///   2. add the current character to the output and skip the abbreviation
struct Solution;

#[derive(Clone)]
struct AbbrText {
    text: Vec<char>,
    idx: usize,
    abbr_count: u32,
}

/// if the final result is a fixed length subset, we can choose in-place change the
/// original text. otherwise we follow a approach growing the result
impl Solution {
    fn unique_generalized_abbreviations(text: String) -> Vec<String> {
        let text = text.chars().collect::<Vec<_>>();
        let mut queue = Vec::new();
        let mut swap_queue = Vec::new();
        queue.push(AbbrText {
            text: Vec::new(),
            idx: 0,
            abbr_count: 0,
        });

        loop {
            while let Some(at) = queue.pop() {
                if at.abbr_count > 0 {
                    // 1. continue abbreviating
                    let mut at_continue = at.clone();
                    at_continue.idx += 1;
                    at_continue.abbr_count += 1;
                    swap_queue.push(at_continue);

                    //2. stop abbreviating, append number and character
                    let mut at_stop = at.clone();
                    at_stop
                        .text
                        .push(char::from_digit(at_stop.abbr_count, 10).unwrap());
                    at_stop.text.push(text[at_stop.idx]);
                    at_stop.idx += 1;
                    at_stop.abbr_count = 0;
                    swap_queue.push(at_stop);
                } else {
                    // 3. start abbreviating
                    let mut at_start = at.clone();
                    at_start.abbr_count += 1;
                    at_start.idx += 1;
                    swap_queue.push(at_start);

                    // 4. directly append character
                    let mut at_direct = at.clone();
                    at_direct.text.push(text[at_direct.idx]);
                    at_direct.idx += 1;
                    swap_queue.push(at_direct);
                }
            }
            std::mem::swap(&mut queue, &mut swap_queue);
            if queue[0].idx == text.len() {
                // stop abbreviating
                queue.iter_mut().for_each(|at| {
                    if at.abbr_count > 0 {
                        at.text.push(char::from_digit(at.abbr_count, 10).unwrap());
                    }
                });
                break;
            }
        }

        queue
            .into_iter()
            .map(|at| String::from_iter(at.text))
            .collect()
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::unique_generalized_abbreviations("BAT".to_string()).len(),
        8
    );
    assert_eq!(
        Solution::unique_generalized_abbreviations("code".to_string()).len(),
        16
    );
}
