use color_eyre::eyre::Result;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;

pub fn initialize_logging() -> Result<()> {
    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::DEBUG.into())
        // .with_env_var("DAEMOS_LOG")
        .from_env_lossy()
        .add_directive("winit=off".parse()?);

    // let env_filter = std::env::var("DAEMOS_LOG")
    //     .ok()
    //     .and_then(|var| EnvFilter::try_new(var).ok())
    //     .unwrap_or_else(|| EnvFilter::new("info"));

    let subscriber = tracing_subscriber::fmt()
        .with_target(false)
        .with_env_filter(env_filter)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    Ok(())
}
