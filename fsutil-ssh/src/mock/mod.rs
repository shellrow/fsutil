//! ## Mock
//!
//! Contains mock for test units

pub mod ssh;
// -- logger

pub fn logger() {
    use std::sync::Once;

    static INIT: Once = Once::new();

    INIT.call_once(|| {
        let subscriber = tracing_subscriber::FmtSubscriber::builder()
            // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
            // will be written to stdout.
            .with_max_level(tracing::Level::DEBUG)
            // completes the builder.
            .finish();
        tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    });
}
