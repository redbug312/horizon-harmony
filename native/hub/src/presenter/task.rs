use std::time::Duration;

use tokio::time::MissedTickBehavior;
use tokio_with_wasm::tokio;

use crate::messages::station::{Schedule, Track};

const REFRESH_COOLDOWN: Duration = Duration::from_secs(1);

pub async fn refresh_track_arrivals() {
    let mut interval = tokio::time::interval(REFRESH_COOLDOWN);
    interval.set_missed_tick_behavior(MissedTickBehavior::Delay);

    loop {
        interval.tick().await;

        let schedule = Schedule {
            station: "公館站".to_owned(),
            tracks: vec![
                Track {
                    destination: "松山站".to_owned(),
                    arrival_time: "營運時間已過".to_owned(),
                },
                Track {
                    destination: "新店站".to_owned(),
                    arrival_time: "營運時間已過".to_owned(),
                },
            ],
            lookup_time: "2024-04-05 01:56:14".to_owned(),
        };
        schedule.send_signal_to_dart(None);
    }
}
