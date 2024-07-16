/// this problem follows the subsets pattern and can be mapped to permutations
///
/// since we need to preserve the character sequence, we can start with the actual
/// string and process each character one by one
///
/// e.g., start with the actual string "ab7c"; processing the first character, we will
/// get two permutations: "ab7c", "Ab7c"; ...
struct Solution;

impl Solution {
    fn string_permutations_by_changing_case(str: String) -> Vec<String> {
        let str = str.chars().collect::<Vec<_>>();
        let mut queue = vec![str.clone()];
        let mut swap_queue = Vec::new();
        for idx in 0..str.len() {
            while let Some(mut variant) = queue.pop() {
                if variant[idx].is_ascii_digit() {
                    swap_queue.push(variant);
                } else {
                    variant[idx] = variant[idx].to_ascii_lowercase();
                    swap_queue.push(variant.clone());
                    variant[idx] = variant[idx].to_ascii_uppercase();
                    swap_queue.push(variant);
                }
            }
            std::mem::swap(&mut queue, &mut swap_queue);
        }
        queue.into_iter().map(String::from_iter).collect()
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::string_permutations_by_changing_case("ad52".to_string()).len(),
        4
    );
    assert_eq!(
        Solution::string_permutations_by_changing_case("ab7c".to_string()).len(),
        8
    );
}
