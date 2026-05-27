pub fn debug(case: &str, i: usize, lptr: Option<i32>, rptr: Option<i32>, h: i32, area: i32) {

    println!("{case} | i: {i:<4?} lptr: {lptr:?} rptr: {rptr:?} h: {h:<4?} area: {area:<4?}");
}

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

        // match (lval.cmp(rval), lval.cmp(left_max), rval.cmp(right_max)) { }

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