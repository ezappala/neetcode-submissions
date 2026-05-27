impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let n = height.len();
        if n < 3 {
            return 0;
        }
    
        let mut left = 0usize;
        let mut right = n - 1;
    
        let mut left_max = 0;
        let mut right_max = 0;
        let mut area = 0;
    
        while left < right {
            match (height.get(left), height.get(right)) {
                (Some(x), Some(y)) if x < y && *x >= left_max => {
                    left_max = *x;
                    left += 1;
                    continue;
                }
                (Some(x), Some(y)) if x < y && *x < left_max => {
                    area += left_max - x;
                    left += 1;
                    continue;
                }
                (Some(x), Some(y)) if x >= y && *y >= right_max => {
                    right_max = *y;
                    right -= 1;
                    continue;
                }
                (Some(x), Some(y)) if x >= y && *y < right_max => {
                    area += right_max - y;
                    right -= 1;
                    continue;
                }
                (None, _) | (_, None) | (_, _) => {}
            }
        }
    
        area
    }
}