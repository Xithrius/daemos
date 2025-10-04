use color_eyre::Result;
use crossbeam::channel::Sender;
use symphonia::core::{audio::SampleBuffer, codecs::Decoder, formats::FormatReader};
use tracing::info;

use crate::pcm::PcmChunk;

const TARGET_FRAMES_PER_CHUNK: usize = 8192;

pub fn decode_producer(
    mut format: Box<dyn FormatReader>,
    mut decoder: Box<dyn Decoder>,
    tx: Sender<PcmChunk>,
) -> Result<()> {
    info!("producer: started");

    let mut produced = 0usize;
    // Target chunking: larger chunks to reduce refill boundaries (~120-170ms @48kHz)
    let mut acc_samples: Vec<f32> = Vec::new();
    let mut acc_channels: Option<u16> = None;
    let mut acc_rate: Option<u32> = None;

    while let Ok(packet) = format.next_packet() {
        let decoded = decoder.decode(&packet)?;
        let spec = *decoded.spec();
        let channel_count = spec.channels.count();

        let mut sample_buf = SampleBuffer::<f32>::new(decoded.capacity() as u64, spec);
        sample_buf.copy_interleaved_ref(decoded);
        let mut samples: Vec<f32> = sample_buf.samples().to_vec();

        let channels = channel_count as u16;
        let rate = spec.rate;

        if acc_channels.is_none() {
            acc_channels = Some(channels);
            acc_rate = Some(rate);
        }
        debug_assert_eq!(acc_channels.unwrap(), channels);
        debug_assert_eq!(acc_rate.unwrap(), rate);

        let frame_width = channels as usize;
        acc_samples.append(&mut samples);

        while acc_samples.len() >= TARGET_FRAMES_PER_CHUNK * frame_width {
            let split_at = TARGET_FRAMES_PER_CHUNK * frame_width;
            let tail = acc_samples.split_off(split_at);
            let out = std::mem::replace(&mut acc_samples, tail);
            let chunk = PcmChunk {
                channels,
                sample_rate: rate,
                samples: out,
            };

            if tx.send(chunk).is_err() {
                return Ok(());
            }

            produced += 1;

            if produced.is_multiple_of(30) {
                info!(
                    "producer: produced={}, ch={}, rate={}",
                    produced, channels, rate
                );
            }
        }
    }

    if !acc_samples.is_empty() {
        let chunk = PcmChunk {
            channels: acc_channels.unwrap_or(2),
            sample_rate: acc_rate.unwrap_or(48000),
            samples: acc_samples,
        };

        let _ = tx.send(chunk);
    }

    info!("producer: finished (eof or consumer dropped)");

    Ok(())
}
