//! This `hub` crate is the
//! entry point of the Rust logic.

use model::api::Api;

// This `tokio` will be used by Rinf.
// You can replace it with the original `tokio`
// if you're not targeting the web.
use tokio_with_wasm::tokio;

mod messages;
mod model;
mod presenter;

rinf::write_interface!();

// Always use non-blocking async functions
// such as `tokio::fs::File::open`.
// If you really need to use blocking code,
// use `tokio::task::spawn_blocking`.
async fn main() {
    // The reqwest client inside the api maintains a connection pool and the
    // docs advises to clone and reuse it across the tasks.
    let api = Api::new();
    tokio::spawn(presenter::task::refresh_track_arrivals(api));
}
