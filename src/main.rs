use serde::Deserialize;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Deserialize)]
pub struct Message<'a> {
    e: &'a str,
    s: &'a str,
    b: &'a str,
    a: &'a str,
    B: &'a str,
    A: &'a str,
}

#[derive(Debug, Clone)]
pub struct StdDepth {
    pub bids: Vec<Vec<f64>>,
    pub asks: Vec<Vec<f64>>,
    pub event_time: i64,
    pub match_time: i64,
    pub local_time_us: u128,
    pub parse_cost: i64,
    pub trans_cost: i64,
}

pub fn timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn main() {
    let mut count = 0;
    let begin = timestamp();
    while timestamp() - begin < 5 {
        let msg = "{\"e\":\"bookTicker\",\"u\":2069831423297,\"s\":\"APTUSDT\",\"b\":\"8.92200\",\"B\":\"98.4\",\"a\":\"8.92300\",\"A\":\"697.7\",\"T\":1666763485645,\"E\":1666763485650}".to_string();
        let messgage = serde_json::from_str::<Message>(&msg).unwrap();
        let mut std_depth = StdDepth {
            bids: vec![vec![
                fast_float::parse(messgage.b).unwrap(),
                fast_float::parse(messgage.B).unwrap(),
            ]],
            asks: vec![vec![
                fast_float::parse(messgage.a).unwrap(),
                fast_float::parse(messgage.A).unwrap(),
            ]],
            event_time: 0,
            match_time: 0,
            parse_cost: 0,
            trans_cost: 0,
            local_time_us: 0,
        };
        std_depth.event_time = 1;
        count += 1;
    }
    println!("total: {}", count);
}
