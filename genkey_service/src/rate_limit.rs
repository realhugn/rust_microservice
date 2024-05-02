use std::collections::HashMap;

use chrono::{NaiveDateTime, Utc};

#[derive(Debug, Clone)]
pub struct SlidingWindow {
    pub window_size: i64,
    pub max_request: i64,
    pub user_window: HashMap<String, RequestTimeStamp>
}

#[derive(Debug, Clone)]
pub struct RequestTimeStamp {
    pub requests : Vec<NaiveDateTime>
}

impl SlidingWindow {
    pub fn new(window_size: i64, max_request: i64) -> Self {
        println!("{}: {}",window_size, max_request);
        Self {
            window_size,
            max_request,
            user_window: HashMap::new(),
        }
    }

    pub fn allow(&mut self, _token: String) -> bool {
        let now = Utc::now();
        let user_window = self.user_window
            .entry(_token)
            .or_insert(
                RequestTimeStamp { requests: Vec::new() }
        );

        let curr_window_key = now.timestamp_millis()/1000 * 1000;
        let mut pre_count = 0;
        let mut curr_count = 0 ;
        let mut req_to_remove: Vec<usize> = Vec::new();
        for i in 0..user_window.requests.len() {
            let rq_time = user_window.requests.get(i).unwrap();
            let time = rq_time.timestamp_millis() ;
            if time < now.timestamp_millis() - self.window_size {
                req_to_remove.push(i);
            } else if time > now.timestamp_millis() - self.window_size && time < curr_window_key {
                pre_count += 1;
            } else {
                curr_count += 1;
            }
        }
        println!("{:?}", req_to_remove);
        for _ in 0..req_to_remove.len() {
            user_window.requests.remove(0);
        }
        
        let pre_weight = 1 - (now.timestamp_millis() - curr_window_key)/1000;
        let count = pre_weight*pre_count  + curr_count + 1 <self.max_request;
        if count {
            user_window.requests.push(now.naive_utc());
            true
        } else {
            false
        }
    }
}