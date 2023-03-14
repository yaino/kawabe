use chrono::{DateTime, Local};

use super::AlertLevel;

pub struct DetectedAlert {
    pub level: AlertLevel,
    pub distance: f32,
    pub detected_date_time: DateTime<Local>,
}

impl DetectedAlert {
    pub fn new(level: AlertLevel, distance: f32, detected_date_time: DateTime<Local>) -> Self {
        Self {
            level,
            distance,
            detected_date_time,
        }
    }
}
