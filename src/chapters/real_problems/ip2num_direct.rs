struct Solution;

impl Solution {
    fn get_ip(raw_ip: &str) -> Option<u64> {
        let group = raw_ip.split('.').map(|seg| seg.trim().parse::<u64>().ok());

        let mut res = 0;
        let mut count = 0;
        for seg in group {
            let seg = seg?;
            if seg > 255 {
                return None;
            } else {
                dbg!(seg);
                res = res * 256 + seg;
                count += 1;
            }
            if count > 4 {
                return None;
            }
        }
        if count == 4 {
            Some(res)
        } else {
            None
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

#[test]
fn test_u32() {
    assert!(255 * 256u64.pow(3) + 255 * 256u64.pow(2) + 255 * 256u64 + 255u64 < u64::MAX)
}

#[test]
fn test_normal() {
    assert_eq!(
        Solution::get_ip("172.168.5.1"),
        Some(172 * 256u64.pow(3) + 168 * 256u64.pow(2) + 5 * 256u64 + 1)
    );
    assert_eq!(Solution::get_ip("172.0.0.0"), Some(172 * 256u64.pow(3)));
    assert_eq!(Solution::get_ip("0.0.0.1"), Some(1u64));
    assert_eq!(Solution::get_ip("0.0.0.0"), Some(0u64));
}
#[test]
fn test_valid_space() {
    assert_eq!(
        Solution::get_ip("172  . 168 . 5. 1"),
        Some(172 * 256u64.pow(3) + 168 * 256u64.pow(2) + 5 * 256u64 + 1)
    );
    assert_eq!(Solution::get_ip(" 172  . 168 . 5. 1 "), Some(2896692481u64));
}
#[test]
fn test_invalid_space() {
    assert_eq!(Solution::get_ip("1 72.168.5.1"), None);
    assert_eq!(Solution::get_ip("172  .16 8.5.1"), None);
    assert_eq!(Solution::get_ip("0.0 0.0.1"), None);
}

#[test]
fn test_invalid_character() {
    assert_eq!(Solution::get_ip("172.a68.5.1"), None);
    assert_eq!(Solution::get_ip("-172.68.5.1"), None);
    assert_eq!(Solution::get_ip("172.1681.5.1"), None);
    assert_eq!(Solution::get_ip("172.256.5.1"), None);
}

#[test]
fn test_invalid_semantic() {
    assert_eq!(Solution::get_ip("172..1.1"), None);
    assert_eq!(Solution::get_ip("172.1.1"), None);
    assert_eq!(Solution::get_ip("172.68.5.1.1"), None);
    assert_eq!(Solution::get_ip("0.0.0.0.0"), None);
    assert_eq!(Solution::get_ip("255.255.255.255.255"), None);
}
// }
