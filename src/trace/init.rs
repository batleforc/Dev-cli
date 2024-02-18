use tracing::subscriber;

pub fn init_tracing(level: tracing::Level) {
    let subs = tracing_subscriber::fmt().with_max_level(level).finish();

    subscriber::set_global_default(subs).expect("setting default subscriber failed");
}
