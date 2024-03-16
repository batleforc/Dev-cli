use ::time::format_description;
use std::{fs::File, sync::Arc};
use tracing::{level_filters::LevelFilter, subscriber};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::fmt::time::UtcTime;
use tracing_subscriber::fmt::{self};
use tracing_subscriber::{layer::SubscriberExt, Layer, Registry};
pub fn init_tracing(level: tracing::Level, trace: bool) {
    let time_format = format_description::parse("[hour]:[minute]:[second]")
        .expect("format string should be valid!");
    let timer = UtcTime::new(time_format);

    let terminal_out = fmt::layer()
        .with_timer(timer)
        .with_target(false)
        .with_filter(LevelFilter::from(level));
    if trace {
        let file = File::options()
            .create(true)
            .truncate(false)
            .append(true)
            .open("trace.log")
            .expect("Failed to create trace.log");
        let formating_layer = BunyanFormattingLayer::new("dev_cli".into(), Arc::new(file));
        subscriber::set_global_default(
            Registry::default()
                .with(terminal_out)
                .with(JsonStorageLayer)
                .with(formating_layer),
        )
        .expect("setting default subscriber failed");
    } else {
        subscriber::set_global_default(Registry::default().with(terminal_out))
            .expect("setting default subscriber failed");
    }
}
