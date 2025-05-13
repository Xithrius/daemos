use serde::{Deserialize, Serialize};

const DEFAULT_PLAYER_VOLUME: f32 = 50.0;

fn clamp_volume<'de, D>(deserializer: D) -> Result<f32, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = f32::deserialize(deserializer)?;

    Ok(value.clamp(0.0, 100.0))
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(default)]
pub struct CoreConfig {
    pub debug: bool,
    pub vsync: bool,
    #[serde(deserialize_with = "clamp_volume")]
    pub volume: f32,
}

impl Default for CoreConfig {
    fn default() -> Self {
        Self {
            debug: Default::default(),
            vsync: Default::default(),
            volume: DEFAULT_PLAYER_VOLUME,
        }
    }
}
