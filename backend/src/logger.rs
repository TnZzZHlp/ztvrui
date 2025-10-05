pub fn init_logger(log_level: &str) {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(log_level.parse().unwrap_or(tracing::Level::INFO))
        .event_format(
            tracing_subscriber::fmt::format()
                .with_target(false)
                .with_file(false)
                .with_line_number(false),
        )
        .compact()
        .init();
}
