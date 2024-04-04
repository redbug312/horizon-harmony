use std::time::Duration;

use rinf::debug_print;
use time::OffsetDateTime;
use tokio::time::MissedTickBehavior;
use tokio_with_wasm::tokio;

use crate::messages::station::{Schedule, Track};
use crate::model::metro::track_info;
use crate::model::api::Api;

const REFRESH_COOLDOWN: Duration = Duration::from_secs(10);

pub async fn refresh_track_arrivals(api: Api) {
    let mut interval = tokio::time::interval(REFRESH_COOLDOWN);
    interval.set_missed_tick_behavior(MissedTickBehavior::Delay);

    loop {
        interval.tick().await;

        let reply = api.fetch_track_info().await;
        let schedule = Schedule::from(reply);
        schedule.send_signal_to_dart(None);
        debug_print!("Refresh track arrivals {schedule:?}");
    }
}

impl From<track_info::Reply> for Schedule {
    fn from(reply: track_info::Reply) -> Self {
        let station = "公館站".to_owned();
        let tracks = reply.0.into_iter()
            .filter(|info| info.station_name == station)
            .map(|info| Track {
                destination: info.destination_name,
                arrival_time: info.count_down,
            })
            .collect();
        let lookup_time = OffsetDateTime::now_utc().to_string();
        Schedule { station, tracks, lookup_time }
    }
}
