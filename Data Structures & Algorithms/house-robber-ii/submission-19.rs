impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let n = nums.len();

        if n == 1 {
            return nums[0];
        }

        fn rob_line(nums: &[i32]) -> i32 {
            let mut prev_two = 0;
            let mut prev_one = 0;

            for &money in nums {
                let rob_this = prev_two + money;
                let skip_this = prev_one;

                let current = rob_this.max(skip_this);

                prev_two = prev_one;
                prev_one = current;
            }

            prev_one
        }

        let without_last = rob_line(&nums[..n - 1]);
        let without_first = rob_line(&nums[1..]);

        without_last.max(without_first)
    }
}
