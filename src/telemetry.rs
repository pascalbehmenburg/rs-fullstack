use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

/// Enables structured logging for easier debugging and monitoring.
/// The logs are output to stdout in a JSON format.
/// actix logs are included by `Tracing::Logger` middleware which attaches uuid's to requests too.
/// Note: The log level can be set using the `RUST_LOG` environment variable.
/// You may tunnel outputs into bunyan which can be installed using `cargo install bunyan`.
#[tracing::instrument("init subscriber")]
pub fn init_subscriber(log_level: tracing::Level, layer_name: String) {
    LogTracer::init().expect("failed to set logger");

    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::from(log_level.to_string()));

    let formatting_layer = BunyanFormattingLayer::new(layer_name, std::io::stdout);

    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);

    set_global_default(subscriber).expect("failed to set subscriber");
}
