use std::{fs::File, sync::Arc};

use tracing::{level_filters::LevelFilter, subscriber};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{layer::SubscriberExt, Layer, Registry};

pub fn init_tracing(level: tracing::Level, trace: bool) {
    let terminal_out = tracing_subscriber::fmt::layer().with_filter(LevelFilter::from(level));
    if trace {
        let file = File::options()
            .create(true)
            .write(true)
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
