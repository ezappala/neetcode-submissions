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
            let (Some(lval), Some(rval)) = (height.get(left), height.get(right)) else {
                panic!("No val")
            };
    
            match (lval, rval) {
                (x, y) if x < y && *x >= left_max => {
                    left_max = *lval;
                    left += 1;
                    continue;
                }
                (x, y) if x < y && *x < left_max => {
                    area += left_max - lval;
                    left += 1;
                    continue;
                }
                (x, y) if x >= y && *y >= right_max => {
                    right_max = *rval;
                    right -= 1;
                    continue;
                }
                (x, y) if x >= y && *y < right_max => {
                    area += right_max - rval;
                    right -= 1;
                    continue;
                }
                (_, _) => {}
            }
        }
    
    
        area
    }
}