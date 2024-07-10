struct ParseError;

struct Ip2Num {
    raw_ip: String,
    pos: usize,
}

impl Iterator for Ip2Num {
    type Item = Result<u32, ParseError>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos == self.raw_ip.len() {
            return None;
        }
        let mut num: u32 = 0;
        let mut space_in_num = 0;
        for (idx, ch) in self.raw_ip[self.pos..].chars().enumerate() {
            if ch == '.' {
                if space_in_num == 0 {
                    return None;
                }
                self.pos = self.pos + idx + 1;
                break;
            } else if ch == ' ' {
                if space_in_num == 1 {
                    space_in_num = 2
                } else {
                    continue;
                }
            } else if let Some(n) = ch.to_digit(10) {
                if space_in_num == 2 {
                    // space in the middle of number
                    return Some(Err(ParseError));
                } else {
                    space_in_num = 1;
                    num = num * 10 + n;
                }
            } else {
                // character not a number
                return Some(Err(ParseError));
            }
        }
        if num < 256 {
            Some(Ok(num))
        } else {
            // larger than 255
            Some(Err(ParseError))
        }
    }
}

impl Ip2Num {
    fn new(raw_ip: &str) -> Self {
        Ip2Num {
            raw_ip: format!("{}.", raw_ip),
            pos: 0,
        }
    }
    fn get_ip(&mut self) -> Option<u64> {
        let mut res = 0;
        let mut count = 0;
        for seg in self {
            if let Ok(num) = seg {
                dbg!(num);
                res = res * 256 + num as u64;
                count += 1;
            } else {
                // Parse error
                return None;
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
// use super::*;

#[test]
fn test_u32() {
    assert!(255 * 256u64.pow(3) + 255 * 256u64.pow(2) + 255 * 256u64 + 255u64 < u64::MAX)
}

#[test]
fn test_valid() {
    assert_eq!(
        Ip2Num::new("255.255.255.255").get_ip(),
        Some(255 * 256u64.pow(3) + 255 * 256u64.pow(2) + 255 * 256u64 + 255)
    );
    assert_eq!(
        Ip2Num::new("172.168.5.1").get_ip(),
        Some(172 * 256u64.pow(3) + 168 * 256u64.pow(2) + 5 * 256u64 + 1)
    );
    assert_eq!(Ip2Num::new("172.0.0.0").get_ip(), Some(172 * 256u64.pow(3)));
    assert_eq!(Ip2Num::new("0.0.0.1").get_ip(), Some(1u64));
    assert_eq!(Ip2Num::new("0.0.0.0").get_ip(), Some(0u64));
}
#[test]
fn test_valid_space() {
    assert_eq!(
        Ip2Num::new("172  . 168 . 5. 1").get_ip(),
        Some(172 * 256u64.pow(3) + 168 * 256u64.pow(2) + 5 * 256u64 + 1)
    );
    assert_eq!(
        Ip2Num::new(" 172  . 168 . 5. 1 ").get_ip(),
        Some(2896692481u64)
    );
}
#[test]
fn test_invalid_space() {
    assert_eq!(Ip2Num::new("1 72.168.5.1").get_ip(), None);
    assert_eq!(Ip2Num::new("172  .16 8.5.1").get_ip(), None);
    assert_eq!(Ip2Num::new("0.0 0.0.1").get_ip(), None);
}

#[test]
fn test_invalid_character() {
    assert_eq!(Ip2Num::new("172.a68.5.1").get_ip(), None);
    assert_eq!(Ip2Num::new("-172.68.5.1").get_ip(), None);
    assert_eq!(Ip2Num::new("172.1681.5.1").get_ip(), None);
    assert_eq!(Ip2Num::new("172.256.5.1").get_ip(), None);
}
#[test]
fn test_invalid_semantic() {
    assert_eq!(Ip2Num::new("172.0.1").get_ip(), None);
    assert_eq!(Ip2Num::new("172.0..1").get_ip(), None);
    assert_eq!(Ip2Num::new("172.68.5.1.1").get_ip(), None);
    assert_eq!(Ip2Num::new("0.0.0.0.0").get_ip(), None);
    assert_eq!(Ip2Num::new("255.255.255.255.255").get_ip(), None);
}
// }
