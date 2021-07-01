use std::collections::HashSet;

fn bool_to_option(b: bool) -> Option<()> {
    if b {
        Some(())
    } else {
        None
    }
}

/// 输入: `s = "abcabcbb"`  
/// 输出: `3`  
/// 解释: `因为无重复字符的最长子串是 "abc"，所以其长度为 3。`
#[allow(dead_code)]
fn length_of_longest_substring(s: String) -> i32 {
    let (chars, len) = (s.as_bytes(), s.len());

    let mut longest_substring_len: usize = 0;

    let mut char_set: HashSet<u8> = HashSet::new();

    (0..len).try_for_each(|i| {
        (i..len)
            .try_for_each(|j| bool_to_option(char_set.insert(chars[j])))
            .and(None)
            .or(bool_to_option(char_set.len() <= longest_substring_len))
            .or_else(|| Some(longest_substring_len = char_set.len()))
            .and_then(|_| {
                char_set.clear();
                bool_to_option(longest_substring_len < len - i - 1)
            })
    });

    longest_substring_len as i32
}

#[cfg(test)]
mod mods {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(length_of_longest_substring(String::from("")), 0);
        assert_eq!(length_of_longest_substring(String::from("aaaa")), 1);
        assert_eq!(length_of_longest_substring(String::from("abcdefg")), 7);
        assert_eq!(length_of_longest_substring(String::from("abac")), 3);
        assert_eq!(length_of_longest_substring(String::from("abca")), 3);
        assert_eq!(length_of_longest_substring(String::from("pwwkew")), 3);
        assert_eq!(length_of_longest_substring(String::from("abcabcbb")), 3);
    }
}
