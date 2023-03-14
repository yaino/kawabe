use std::{
    collections::VecDeque,
    sync::mpsc::{Receiver, Sender},
    thread,
};

mod alert_level;
pub use alert_level::AlertLevel;

mod detected_alert;
use chrono::Local;
pub use detected_alert::DetectedAlert;

#[derive(Debug)]
struct Threshold {
    pub level: AlertLevel,
    pub lower_distance: f32,
    pub upper_distance: f32,
}

const TABLE: [Threshold; 4] = [
    Threshold {
        level: AlertLevel::Danger,
        lower_distance: 0.0,
        upper_distance: 13.0,
    },
    Threshold {
        level: AlertLevel::Warning,
        lower_distance: 7.0,
        upper_distance: 21.0,
    },
    Threshold {
        level: AlertLevel::Caution,
        lower_distance: 15.0,
        upper_distance: 29.0,
    },
    Threshold {
        level: AlertLevel::Safety,
        lower_distance: 23.0,
        upper_distance: 999.0,
    },
];

#[derive(Debug)]
pub struct DetectWorker {
    pub level: AlertLevel,
    pub distance: f32,
    pub queue: VecDeque<f32>,
}

impl DetectWorker {
    pub fn new() -> Self {
        Self {
            level: AlertLevel::Safety,
            distance: 0.0,
            queue: VecDeque::new(),
        }
    }
    pub fn run(mut self, sensor_rx: Receiver<f32>, worker_tx: Sender<DetectedAlert>) {
        thread::spawn(move || {
            loop {
                while let Ok(distance) = sensor_rx.recv() {
                    self.queue.push_back(distance);
                    let distance = if self.queue.len() >= 5 {
                        self.queue.pop_front();
                        let sum: f32 = self.queue.iter().fold(0.0, |sum, i| sum + i);
                        sum / 4.0
                    } else {
                        999.0
                    };
                    // println!("distance = {} cm", distance);
                    let previous_level = self.level.clone();

                    // 今の警告レベルに対してのしきい値上限下限を取得する
                    let threshold = TABLE.iter().find(|tbl| tbl.level == self.level);
                    self.level = match threshold {
                        // しきい値超えあり
                        Some(th) => {
                            // 上限しきい値を超えていたら警告レベルをどこに引き上げるかを判定
                            if th.upper_distance < distance {
                                TABLE
                                    .into_iter()
                                    .find(|tbl| tbl.upper_distance > distance)
                                    .map_or(self.level, |th| th.level)
                            // 下限しきい値を超えていたら警告レベルをどこまで引き下げるかを判定
                            } else if th.lower_distance > distance {
                                TABLE
                                    .into_iter()
                                    .rev()
                                    .find(|tbl| tbl.lower_distance < distance)
                                    .map_or(self.level, |th| th.level)
                            } else {
                                self.level
                            }
                        }
                        // しきい値超え無し
                        None => self.level,
                    };

                    if previous_level != self.level {
                        // level を送信
                        worker_tx
                            .send(DetectedAlert::new(
                                self.level.clone(),
                                distance,
                                Local::now(),
                            ))
                            .unwrap();
                    }
                    self.distance = distance;
                }
            }
        });
    }
}
