use std::{time::{SystemTime, UNIX_EPOCH}, collections::{HashMap, VecDeque}};
// use diesel::prelude::*;
use chrono::{NaiveDateTime, Local, TimeZone, Utc};
// use diesel::PgConnection;

// use crate::{model::{AccessRequest, NewLog, UpdateLog}, schema::access_request::user_id};

// const MAX_TOKENS: usize = 2;
// const RATE_MS: u64 = 5000; // ms

// type DbError = Box<dyn std::error::Error + Send + Sync>;

// #[derive(Clone, Copy)]
// pub struct TokenBucket {
//     user_id : i32,
//     current: usize,
//     last_refill_time:usize,
// }

// impl TokenBucket {
//     pub fn new(_user_id :i32, _current: usize, _last_refill_time: NaiveDateTime) -> Self {
//         let date_time = Local.from_local_datetime(&_last_refill_time).unwrap();
//         TokenBucket{
//             user_id: _user_id,  
//             current: _current,
//             last_refill_time: SystemTime::from(date_time).duration_since(UNIX_EPOCH).unwrap().as_millis() as usize
//         }
//     }
// }

// pub fn new_bucket_for_uid (_user_id: i32, conn :&mut PgConnection) -> Result<TokenBucket, DbError>{
//     use crate::schema::access_request::dsl::*;
//     let exist: Option<AccessRequest> = access_request
//         .filter(user_id.eq(_user_id))
//         .first::<AccessRequest>(conn)
//         .optional()?;
//     match exist {
//         Some(access_info) => {
//             let token = TokenBucket::new(access_info.user_id, access_info.count as usize, access_info.time);
//             Ok(token)
//         }, 
//         None => {
//             let new = NewLog {
//                 user_id: _user_id,
//                 count: MAX_TOKENS as i32
//             };  
//             let access_info = diesel::insert_into(access_request).values(&new).get_result::<AccessRequest>(conn)?;
//             let token = TokenBucket::new(access_info.user_id, access_info.count as usize, access_info.time);
//             Ok(token)
//         }
//     }
// }

// pub fn consume(bucket: &mut TokenBucket, conn: &mut PgConnection) -> Result<bool, DbError> {
//     use crate::schema::access_request::dsl::*;
//     let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
//     let elapsed = now as usize;
//     let refill_amount = (elapsed - bucket.last_refill_time) / RATE_MS as usize;
//     if refill_amount > 0 {
//         let current_tokens = bucket.current + refill_amount;
//         let max_tokens = MAX_TOKENS.min(current_tokens);
//         let upd = UpdateLog {
//             count : max_tokens as i32,
//             time: Utc::now().naive_utc()
//         };
//         let _ = diesel::update(access_request.find(bucket.user_id))
//             .set(upd)
//             .get_result::<AccessRequest>(conn); 
//         bucket.current = max_tokens ;
//     }
//     let available_tokens =  bucket.current;
//     if available_tokens > 0 {
//         let upd = UpdateLog {
//             count : (available_tokens - 1 ) as i32,
//             time: Utc::now().naive_utc()
//         };
//         let _ = diesel::update(access_request.find(bucket.user_id))
//             .set(upd)
//             .get_result::<AccessRequest>(conn); 
//         Ok(true)
//     } else {
//         Ok(false)
//     }
// }

#[derive(Debug, Clone)]
pub struct SlidingWindow {
    pub window_size: i64,
    pub max_request: i64,
    pub user_window: HashMap<i32, RequestTimeStamp>
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

    pub fn allow(&mut self, _user_id: i32) -> bool {
        let now = Utc::now();
        let user_window = self.user_window
            .entry(_user_id)
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
        for i in 0..req_to_remove.len() {
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