pub mod cache;
pub mod client;
pub mod constants;
pub mod core;
pub mod database;
pub mod errors;
pub mod events;
pub mod interactions;
pub mod macros;
pub mod owner;
pub mod utils;

use snafu::ErrorCompat;
use tokio::main;
use tracing_subscriber::{EnvFilter, fmt};

use crate::client::{bot::StarboardBot, config::Config, runner::run};

fn init_tracing() {
    tracing::subscriber::set_global_default(
        fmt::Subscriber::builder()
            .with_env_filter(EnvFilter::from_default_env())
            .finish(),
    )
    .expect("Unable to set global tracing subscriber");
}

#[main]
async fn main() {
    init_tracing();

    let config = Config::from_env();

    let bot = match StarboardBot::new(config).await {
        Ok(val) => val,
        Err(why) => {
            eprintln!("{}", why);
            if let Some(bt) = ErrorCompat::backtrace(&why) {
                eprintln!("{:#?}", bt);
            }
            return;
        }
    };

    tracing::info!("READY");

    run(bot).await;
}
