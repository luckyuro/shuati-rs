use super::Solution;

impl Solution {
    pub fn average_waiting_time(customers: Vec<Vec<i32>>) -> f64 {
        let mut wait_time = 0f64;
        let mut finish_time = 0;
        for custom in customers.iter() {
            let (arrival, time) = (custom[0], custom[1]);
            if arrival > finish_time {
                finish_time = arrival + time;
            } else {
                finish_time += time;
            }
            wait_time += (finish_time - arrival) as f64;
        }
        wait_time / customers.len() as f64
    }
}
