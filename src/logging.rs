use color_eyre::eyre::Result;
use tracing_subscriber::EnvFilter;

pub fn initialize_logging() -> Result<()> {
    let env_filter = std::env::var("DRAKN_LOG")
        .ok()
        .and_then(|var| EnvFilter::try_new(var).ok())
        .unwrap_or_else(|| EnvFilter::new("info"));

    let subscriber = tracing_subscriber::fmt()
        .with_target(false)
        .with_env_filter(env_filter)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    Ok(())
}
