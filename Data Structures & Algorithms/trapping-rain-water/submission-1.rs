pub fn debug(case: &str, i: usize, lptr: Option<i32>, rptr: Option<i32>, h: i32, area: i32) {

    println!("{case} | i: {i:<4?} lptr: {lptr:?} rptr: {rptr:?} h: {h:<4?} area: {area:<4?}");
}

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        if height.len() < 3 {
            return 0;
        }

        let mut left = 0usize;
        let mut right = height.len() - 1;

        let mut left_max = 0;
        let mut right_max = 0;
        let mut area = 0;

        while left < right {
            if height[left] < height[right] {
                if height[left] >= left_max {
                    left_max = height[left];
                } else {
                    area += left_max - height[left];
                }

                left += 1;
            } else {
                if height[right] >= right_max {
                    right_max = height[right];
                } else {
                    area += right_max - height[right];
                }

                right -= 1;
            }
        }

        area
    }
}