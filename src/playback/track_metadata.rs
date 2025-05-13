use std::{fs::File, time::Duration};

use color_eyre::{Result, eyre::ContextCompat};
use symphonia::{
    core::{
        audio::Channels,
        codecs::{CODEC_TYPE_NULL, CodecParameters},
        formats::FormatOptions,
        io::MediaSourceStream,
        meta::MetadataOptions,
    },
    default::get_probe,
};

pub fn extract_track_metadata(file_path: &str) -> Result<CodecParameters> {
    let file = File::open(file_path).expect("Failed to open file");
    let mss = MediaSourceStream::new(Box::new(file), Default::default());

    let probed = get_probe()
        .format(
            &Default::default(),
            mss,
            &FormatOptions::default(),
            &MetadataOptions::default(),
        )
        .expect("Unsupported format or read error");

    let format = probed.format;
    let track = format
        .tracks()
        .iter()
        .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
        .context(format!("No supported audio for track {:?}", file_path))?;

    Ok(track.codec_params.clone())
}

pub fn extract_track_duration(codec_params: CodecParameters) -> Option<Duration> {
    if let (Some(sr), Some(frames)) = (codec_params.sample_rate, codec_params.n_frames) {
        let duration = Duration::from_secs_f64(frames as f64 / sr as f64);

        Some(duration)
    } else {
        None
    }
}

#[allow(dead_code)]
pub fn extract_track_sample_rate(codec_params: CodecParameters) -> Option<u32> {
    codec_params.sample_rate
}

#[allow(dead_code)]
pub fn extract_track_channels(codec_params: CodecParameters) -> Option<Channels> {
    codec_params.channels
}
