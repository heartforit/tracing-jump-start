use std::thread;
use std::time::Duration;
use tracing::{error, info};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::Layer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub fn init() {
    let mut layers = Vec::new();

    // We want a different format for log files (in this case json) to use
    // tooling like:
    // - https://github.com/01mf02/jaq
    // - https://github.com/KunalSin9h/livejq
    // for analyzing the logs on the terminal for example
    let file = tracing_appender::rolling::daily("./logs", "error");
    let layer_file = tracing_subscriber::fmt::layer()
        .with_thread_names(false)
        .with_target(true)
        .with_file(true)
        .with_line_number(true)
        .json()
        .with_writer(file)
        .with_filter(LevelFilter::ERROR)
        .boxed();
    layers.push(layer_file);


    // we want a format for console out put which is readable and pretty for the console
    // but mostly used for development cases
    let layer_stdout = tracing_subscriber::fmt::layer()
        .pretty()
        .with_filter(LevelFilter::TRACE)
        .boxed();
    layers.push(layer_stdout);

    tracing_subscriber::registry()
        .with(layers)
        .init();
}

fn some_error() -> Result<(), anyhow::Error> {
    Err(anyhow::anyhow!("some error"))
}

fn main() {
    init();
    // will end up on terminal but not log file
    info!( user_id = 1, "should be only readable on console");

    // this will only be written to the log file as json and in case of log level error
    let some_error_result: Result<(), anyhow::Error> = some_error();
    error!(trace_id = "trace_id", error = ?some_error_result);

    thread::sleep(Duration::from_secs(2));

}
