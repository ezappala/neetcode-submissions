use std::collections::HashMap;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        if t.is_empty() {
            return String::new();
        }
    
        let s = s.as_bytes();
        let mut count_t: HashMap<u8, i32> = HashMap::new();
        t.as_bytes().iter().for_each(|&c| {
            *count_t.entry(c).or_insert(0) += 1;
        });
    
        let mut window: HashMap<u8, i32> = HashMap::new();
        let mut have = 0;
        let need = count_t.len();
        let mut res = (0usize, 0usize);
        let mut res_len = usize::MAX;
        let mut l = 0;
    
        (0..s.len()).for_each(|r| {
            let c = s[r];
            *window.entry(c).or_insert(0) += 1;
    
            match count_t.get(&c) {
                Some(&required) if window[&c] == required => {
                    have += 1;
                }
                _ => (),
            }
    
            while have == need {
                if (r - l + 1) < res_len {
                    res = (l, r);
                    res_len = r - l + 1;
                }
    
                let left_char = s[l];
                *window.get_mut(&left_char).unwrap() -= 1;
                match count_t.get(&left_char) {
                    Some(&required) if window[&left_char] < required => {
                        have -= 1;
                    }
                    _ => (),
                }
                l += 1;
            }
        });
    
        if res_len == usize::MAX {
            String::new()
        } else {
            String::from_utf8(s[res.0..=res.1].to_vec()).unwrap()
        }
    }
}