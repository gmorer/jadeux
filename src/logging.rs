use flexi_logger::{style, DeferredNow, LogSpecification, Logger};
use log::{Level, LevelFilter};
use std::io::Write;

pub fn init(verbosity: usize) {
    let log_specification = match verbosity {
        0 => LogSpecification::builder()
            .default(LevelFilter::Info)
            .build(),
        1 => LogSpecification::builder()
            .default(LevelFilter::Debug)
            .build(),
        _ => LogSpecification::builder()
            .default(LevelFilter::Trace)
            .build(),
    };
    Logger::with(log_specification)
        .format(format_log_entry)
        .start()
        .unwrap();
}

/// Formats a log entry with color
fn format_log_entry(
    w: &mut dyn Write,
    now: &mut DeferredNow,
    record: &log::Record,
) -> std::io::Result<()> {
    let msg = record.args().to_string();
    let level = record.level();
    let (h, m, s) = now.now().time().as_hms();
    write!(
        w,
        "[ {} ] {}:{}:{} {}",
        style(level).paint(level.to_string()),
        h,
        m,
        s,
        msg
    )
}
