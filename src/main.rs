mod detect_worker;
mod led;
mod log_storage;
mod ultrasonic_sensor;
use crate::{detect_worker::DetectWorker, led::Led, ultrasonic_sensor::UltrasonicSensor};
use std::sync::mpsc;
mod line_notify;

const GPIO_GREEN_LED: u8 = 23;
const GPIO_YELLOW_LED: u8 = 17;
const GPIO_RED_LED: u8 = 27;
const TRIG_PIN: u8 = 15;
const ECHO_PIN: u8 = 14;

fn main() {
    println!("Ultrasonic sense detector");
    let sensor = UltrasonicSensor::new(TRIG_PIN, ECHO_PIN).unwrap();
    let worker = DetectWorker::new();

    let (sensor_tx, sensor_rx) = mpsc::channel();
    let (worker_tx, worker_rx) = mpsc::channel();
    // let (notifier_tx, notifier_rx) = mpsc::channel();

    sensor.run(sensor_tx);
    worker.run(sensor_rx, worker_tx);
    let mut caution_light = Led::new(GPIO_GREEN_LED).unwrap();
    let mut warning_light = Led::new(GPIO_YELLOW_LED).unwrap();
    let mut danger_light = Led::new(GPIO_RED_LED).unwrap();

    loop {
        while let Ok(detected_alert) = worker_rx.recv() {
            match detected_alert.level {
                detect_worker::AlertLevel::Safety => {
                    caution_light.light_off();
                    warning_light.light_off();
                    danger_light.light_off();
                }
                detect_worker::AlertLevel::Caution => {
                    caution_light.light_on();
                    warning_light.light_off();
                    danger_light.light_off();
                }
                detect_worker::AlertLevel::Warning => {
                    caution_light.light_off();
                    warning_light.light_on();
                    danger_light.light_off();
                }
                detect_worker::AlertLevel::Danger => {
                    caution_light.light_off();
                    warning_light.light_off();
                    danger_light.light_on();
                }
            }
            // Line に通知
            line_notify::send(detected_alert.level.clone(), detected_alert.distance);
            // ログ保存
            log_storage::send(
                detected_alert.level.id(),
                (detected_alert.distance * 10.0) as u32,
            );
        }
    }
}
