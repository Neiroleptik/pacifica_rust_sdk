use tracing_subscriber::{EnvFilter, FmtSubscriber};
static INIT: std::sync::Once = std::sync::Once::new();

pub fn init_logging_once(level: &str) {
    INIT.call_once(|| {
        let filter = EnvFilter::new(level);
        FmtSubscriber::builder().with_env_filter(filter).init();
    });
}
