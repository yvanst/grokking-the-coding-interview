/// to coompare the given strings, first, we need to apply the backspaces. an efficient
/// way to do this would be from the end of both the strings. we can have separate
/// pointers, pointing to the last element of the given strings. we can start comparing
/// the characters pointed out by both the pointers to see if the strings ar equal. if,
/// at any stage, the character pointed out by any of the pointers is a backspace(#), we
/// will skip and apply the backspace until we have a valid character available for
/// comparison
struct Solution;

impl Solution {
    fn comparing_strings_containing_backspaces(s1: String, s2: String) -> bool {
        let s1 = s1.chars().collect::<Vec<_>>();
        let s2 = s2.chars().collect::<Vec<_>>();
        let mut idx1 = (s1.len() - 1) as i32;
        let mut idx2 = (s2.len() - 1) as i32;

        while idx1 >= 0 || idx2 >= 0 {
            // in order to deal with the beginning situation, we didn't do the -1 in
            // the valid_idx function
            idx1 = Self::valid_idx(&s1, idx1);
            idx2 = Self::valid_idx(&s2, idx2);
            // potentially change idx1 or idx2, re-check
            if idx1 < 0 && idx2 < 0 {
                return true;
            }
            if idx1 < 0 || idx2 < 0 {
                return false;
            }
            if s1[idx1 as usize] != s2[idx2 as usize] {
                return false;
            }
            // next state
            idx1 -= 1;
            idx2 -= 1;
        }

        true
    }

    fn valid_idx(s: &[char], mut idx: i32) -> i32 {
        let mut backspace_count = 0;
        while idx >= 0 {
            match s[idx as usize] {
                '#' => {
                    backspace_count += 1;
                    idx -= 1;
                }
                _ch if backspace_count > 0 => {
                    backspace_count -= 1;
                    idx -= 1;
                }
                _ch => break,
            }
        }
        idx
    }
}
#[test]
fn test() {
    assert!(Solution::comparing_strings_containing_backspaces(
        "xy#z".to_string(),
        "xzz#".to_string()
    ));

    assert!(!Solution::comparing_strings_containing_backspaces(
        "xy#z".to_string(),
        "xyz#".to_string()
    ));

    assert!(Solution::comparing_strings_containing_backspaces(
        "xp#".to_string(),
        "xyz##".to_string()
    ));

    assert!(Solution::comparing_strings_containing_backspaces(
        "xywrrmp".to_string(),
        "xywrrmu#p".to_string()
    ));
}
