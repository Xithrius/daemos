use color_eyre::eyre::Result;
use tracing_subscriber::EnvFilter;

pub fn initialize_logging() -> Result<()> {
    let env_filter = EnvFilter::builder()
        .with_env_var("DRAKN_LOG")
        .from_env_lossy();

    let subscriber = tracing_subscriber::fmt()
        .with_target(false)
        .with_env_filter(env_filter)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    Ok(())
}
