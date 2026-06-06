/**
 * Definition of Interval:
 * #[derive(Debug, Clone)]
 * pub struct Interval {
 *     pub start: i32,
 *     pub end: i32,
 * }
 *
 * impl Interval {
 *     pub fn new(start: i32, end: i32) -> Self {
 *         Interval { start, end }
 *     }
 * }
 */

use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
   pub fn min_meeting_rooms(mut intervals: Vec<Interval>) -> i32 {
        if intervals.is_empty() {
            return 0;
        }
    
        intervals.sort_by_key(|i| i.start);
    
        let mut rooms: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
    
        for interval in intervals {
            if let Some(&Reverse(earliest_end)) = rooms.peek() {
                if earliest_end <= interval.start {
                    rooms.pop();
                }
            }
    
            rooms.push(Reverse(interval.end));
        }
    
        rooms.len() as i32
    }
}
