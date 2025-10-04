mod decoder;
mod http_source;
mod pcm;

use std::{collections::VecDeque, thread, time::Duration};

use color_eyre::Result;
use crossbeam::channel;
use daemos_core::logging::initialize_logging_with_default;
use rodio::{OutputStream, Sink};
use symphonia::{
    core::{
        codecs::DecoderOptions, formats::FormatOptions, io::MediaSourceStream,
        meta::MetadataOptions,
    },
    default::{get_codecs, get_probe},
};
use tracing::info;

use crate::{
    decoder::decode_producer,
    http_source::request_http_stream,
    pcm::{PcmChunk, PcmSource},
};

const TRACK_URL: &str = "http://127.0.0.1:7070/track";

const PRE_BUFFER_CHUNKS: usize = 4;

fn main() -> Result<()> {
    initialize_logging_with_default("DAEMOS_HEADLESS_LOG").expect("Failed to initialize logger");

    info!("http: starting request to /track");
    let source = request_http_stream(TRACK_URL)?;
    // Setup Symphonia decoder with streaming source
    let mss = MediaSourceStream::new(Box::new(source), Default::default());

    let probed = get_probe().format(
        &Default::default(),
        mss,
        &FormatOptions::default(),
        &MetadataOptions::default(),
    )?;
    info!("probe: format detected");

    let format = probed.format;
    let track = format.default_track().expect("No default track");
    info!("track: codec={:?}", track.codec_params.codec);

    let decoder = get_codecs().make(&track.codec_params, &DecoderOptions { verify: false })?;
    info!("decoder: initialized (verify=false)");

    // Setup Rodio output
    let (_stream, handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&handle)?;

    // Bounded producer-consumer: decoder thread -> sync_channel -> sink
    let (tx, rx) = channel::bounded::<PcmChunk>(64);
    info!("pipeline: channel capacity=64");

    info!("producer: thread spawn");
    thread::spawn(move || {
        // Move format and decoder into producer thread
        let _ = decode_producer(format, decoder, tx);
    });

    // Initialize a blocking Source from the first chunk to set format, then stream on demand
    let first = rx.recv().expect("producer ended before first chunk");
    info!(
        "consumer: first chunk ch={}, rate={}, samples={}",
        first.channels,
        first.sample_rate,
        first.samples.len()
    );

    // Pre-roll: accumulate a few more chunks before starting playback to avoid initial underrun
    let mut prebuffer: VecDeque<f32> = first.samples.into();
    let mut added = 0usize;

    while added < PRE_BUFFER_CHUNKS {
        match rx.recv_timeout(Duration::from_millis(30)) {
            Ok(mut more) => {
                if more.channels == first.channels && more.sample_rate == first.sample_rate {
                    prebuffer.extend(more.samples.drain(..));
                    added += 1;
                } else {
                    // Unexpected format change; stop preread to not mix formats
                    break;
                }
            }
            Err(_) => break,
        }
    }

    let src = PcmSource::new(rx, prebuffer, first.channels, first.sample_rate);

    sink.append(src);
    sink.set_volume(0.5);

    sink.sleep_until_end();
    info!("main: playback finished");
    Ok(())
}
