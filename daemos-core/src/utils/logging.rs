use color_eyre::eyre::Result;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;

pub fn initialize_logging(env_filter: EnvFilter) -> Result<()> {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_env_filter(env_filter)
        .init();

    Ok(())
}

pub fn initialize_logging_with_default(env_var: &str) -> Result<()> {
    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .with_env_var(env_var)
        .from_env_lossy();

    initialize_logging(env_filter)
}
