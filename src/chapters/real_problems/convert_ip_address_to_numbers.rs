struct Solution {
    ip: String,
}

impl Solution {
    fn new(ip: &str) -> Self {
        Self { ip: ip.to_string() }
    }
    fn get_ip(&self) -> Option<u64> {
        let group = self
            .ip
            .split('.')
            .map(|seg| seg.trim().parse::<u8>().ok())
            .collect::<Option<Vec<_>>>()?;

        if group.len() != 4 {
            return None;
        }

        group
            .into_iter()
            .map(|e| e as u64)
            .reduce(|acc, e| acc * 256 + e)
    }
}

struct SolutionIterator {
    ip: Vec<char>,
    pos: usize,
}

impl Iterator for SolutionIterator {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos == self.ip.len() {
            return None;
        }
        let mut res = None;
        let mut encounter_space = false;
        for idx in self.pos..self.ip.len() {
            let ch = self.ip[idx];
            match ch {
                '.' => {
                    self.pos = idx + 1;
                    break;
                }
                ' ' => {
                    if res.is_some() {
                        encounter_space = true;
                    }
                }
                ch => {
                    if let Some(d) = ch.to_digit(10) {
                        if encounter_space {
                            return None;
                        }
                        if res.is_none() {
                            res = Some(0);
                        }
                        res = res.map(|acc| acc * 10 + d);
                    } else {
                        return None;
                    }
                }
            }
            self.pos = idx + 1;
        }

        res.and_then(|n| if n < 256 { Some(n as u8) } else { None })
    }
}

impl SolutionIterator {
    fn new(ip: &str) -> Self {
        SolutionIterator {
            ip: ip.chars().collect(),
            pos: 0,
        }
    }
    fn get_ip(&mut self) -> Option<u64> {
        let nums = self.by_ref().collect::<Vec<_>>();
        if nums.len() != 4 {
            return None;
        }
        nums.into_iter()
            .map(|e| e as u64)
            .reduce(|acc, e| acc * 256 + e)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_u32() {
        assert!(255 * 256u64.pow(3) + 255 * 256u64.pow(2) + 255 * 256u64 + 255u64 < u64::MAX)
    }

    #[test]
    fn test_valid() {
        assert_eq!(
            Solution::new("255.255.255.255").get_ip(),
            Some(255 * 256u64.pow(3) + 255 * 256u64.pow(2) + 255 * 256u64 + 255)
        );
        assert_eq!(
            Solution::new("172.168.5.1").get_ip(),
            Some(172 * 256u64.pow(3) + 168 * 256u64.pow(2) + 5 * 256u64 + 1)
        );
        assert_eq!(
            Solution::new("172.0.0.0").get_ip(),
            Some(172 * 256u64.pow(3))
        );
        assert_eq!(Solution::new("0.0.0.1").get_ip(), Some(1u64));
        assert_eq!(Solution::new("0.0.0.0").get_ip(), Some(0u64));
    }
    #[test]
    fn test_valid_space() {
        assert_eq!(
            Solution::new("172  . 168 . 5. 1").get_ip(),
            Some(172 * 256u64.pow(3) + 168 * 256u64.pow(2) + 5 * 256u64 + 1)
        );
        assert_eq!(
            Solution::new(" 172  . 168 . 5. 1 ").get_ip(),
            Some(2896692481u64)
        );
    }
    #[test]
    fn test_invalid_space() {
        assert_eq!(Solution::new("1 72.168.5.1").get_ip(), None);
        assert_eq!(Solution::new("172  .16 8.5.1").get_ip(), None);
        assert_eq!(Solution::new("0.0 0.0.1").get_ip(), None);
    }

    #[test]
    fn test_invalid_character() {
        assert_eq!(Solution::new("172.a68.5.1").get_ip(), None);
        assert_eq!(Solution::new("-172.68.5.1").get_ip(), None);
        assert_eq!(Solution::new("172.1681.5.1").get_ip(), None);
        assert_eq!(Solution::new("172.256.5.1").get_ip(), None);
    }
    #[test]
    fn test_invalid_semantic() {
        assert_eq!(Solution::new("172.0.1").get_ip(), None);
        assert_eq!(Solution::new("172.0..1").get_ip(), None);
        assert_eq!(Solution::new("172.68.5.1.1").get_ip(), None);
        assert_eq!(Solution::new("0.0.0.0.0").get_ip(), None);
        assert_eq!(Solution::new("255.255.255.255.255").get_ip(), None);
    }
}

#[cfg(test)]
mod test_iterator {
    use super::*;

    #[test]
    fn test_u64() {
        assert!(255 * 256u64.pow(3) + 255 * 256u64.pow(2) + 255 * 256u64 + 255u64 < u64::MAX)
    }

    #[test]
    fn test_valid() {
        assert_eq!(
            SolutionIterator::new("255.255.255.255").get_ip(),
            Some(255 * 256u64.pow(3) + 255 * 256u64.pow(2) + 255 * 256u64 + 255)
        );
        assert_eq!(
            SolutionIterator::new("172.168.5.1").get_ip(),
            Some(172 * 256u64.pow(3) + 168 * 256u64.pow(2) + 5 * 256u64 + 1)
        );
        assert_eq!(
            SolutionIterator::new("172.0.0.0").get_ip(),
            Some(172 * 256u64.pow(3))
        );
        assert_eq!(SolutionIterator::new("0.0.0.1").get_ip(), Some(1u64));
        assert_eq!(SolutionIterator::new("0.0.0.0").get_ip(), Some(0u64));
    }
    #[test]
    fn test_valid_space() {
        assert_eq!(
            SolutionIterator::new("172  . 168 . 5. 1").get_ip(),
            Some(172 * 256u64.pow(3) + 168 * 256u64.pow(2) + 5 * 256u64 + 1)
        );
        assert_eq!(
            SolutionIterator::new(" 172  . 168 . 5. 1 ").get_ip(),
            Some(2896692481u64)
        );
    }
    #[test]
    fn test_invalid_space() {
        assert_eq!(SolutionIterator::new("1 72.168.5.1").get_ip(), None);
        assert_eq!(SolutionIterator::new("172  .16 8.5.1").get_ip(), None);
        assert_eq!(SolutionIterator::new("0.0 0.0.1").get_ip(), None);
    }

    #[test]
    fn test_invalid_character() {
        assert_eq!(SolutionIterator::new("172.a68.5.1").get_ip(), None);
        assert_eq!(SolutionIterator::new("-172.68.5.1").get_ip(), None);
        assert_eq!(SolutionIterator::new("172.1681.5.1").get_ip(), None);
        assert_eq!(SolutionIterator::new("172.256.5.1").get_ip(), None);
    }
    #[test]
    fn test_invalid_semantic() {
        assert_eq!(SolutionIterator::new("172.0.1").get_ip(), None);
        assert_eq!(SolutionIterator::new("172.0..1").get_ip(), None);
        assert_eq!(SolutionIterator::new("172.68.5.1.1").get_ip(), None);
        assert_eq!(SolutionIterator::new("0.0.0.0.0").get_ip(), None);
        assert_eq!(SolutionIterator::new("255.255.255.255.255").get_ip(), None);
    }
}
