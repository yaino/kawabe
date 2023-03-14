use std::thread;

use crate::detect_worker::AlertLevel;

pub fn send(alert_level: AlertLevel, distance: f32) {
    thread::spawn(move || {
        let (sticker_package_id, sticker_id) = match alert_level {
            AlertLevel::Safety => (8515, 16581242),
            AlertLevel::Caution => (8515, 16581263),
            AlertLevel::Warning => (446, 2016),
            AlertLevel::Danger => (446, 2018),
        };

        let url = "https://notify-api.line.me/api/notify";
        ureq::post(url)
            .set("Authorization", "Bearer {発行したトークンを記述する}")
            .send_form(&[
                (
                    "message",
                    format!("\n物体を検出しました！\n物体までの距離は{distance:.1}cmです。")
                        .as_str(),
                ),
                ("stickerPackageId", sticker_package_id.to_string().as_str()),
                ("stickerId", sticker_id.to_string().as_str()),
            ])
            .unwrap();
    });
}
