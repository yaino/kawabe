use std::thread;

use chrono::Local;
use serde::Serialize;
use ureq::json;

#[derive(Serialize, Debug)]
struct RequestBody {
    pub id: String,
    pub state: i8,
    pub distance: u32,
    pub detected_at: String,
}

pub fn send(state: i8, distance: u32) {
    thread::spawn(move || {
        let url = "{AWS API GatewayへのURL}";
        let body = RequestBody {
            id: uuid::Uuid::new_v4().to_string(),
            state,
            distance,
            detected_at: Local::now().to_rfc3339(),
        };
        ureq::put(url).send_json(json!(body)).unwrap();
    });
}
