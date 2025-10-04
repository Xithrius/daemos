mod decoder;
mod http_source;
mod pcm;

use std::{collections::VecDeque, thread, time::Duration};

use color_eyre::Result;
use crossbeam::channel;
use daemos_core::logging::initialize_logging_with_default;
use parking_lot::Mutex;
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
    http_source::HttpSource,
    pcm::{PcmChunk, PcmSource},
};

fn main() -> Result<()> {
    initialize_logging_with_default("DAEMOS_HEADLESS_LOG").expect("Failed to initialize logger");

    // HTTP stream setup
    info!("http: starting request to /track");
    let client = reqwest::blocking::ClientBuilder::new()
        .timeout(None)
        .build()?;
    let resp = client
        .get("http://127.0.0.1:7070/track")
        .header(reqwest::header::ACCEPT, "*/*")
        .send()?
        .error_for_status()?;
    info!("http: connected, status {}", resp.status());

    // Setup Symphonia decoder with streaming source
    let source = HttpSource {
        inner: Mutex::new(resp),
    };
    let mss = MediaSourceStream::new(Box::new(source), Default::default());

    let probed = get_probe().format(
        &Default::default(),
        mss,
        &FormatOptions::default(),
        &MetadataOptions::default(),
    )?;
    info!("probe: format detected");

    let format = probed.format;
    let track = format.default_track().unwrap();
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

    while added < 4 {
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

    let src = PcmSource {
        rx,
        buffer: prebuffer,
        channels: first.channels,
        sample_rate: first.sample_rate,
    };

    sink.append(src);

    sink.sleep_until_end();
    info!("main: playback finished");
    Ok(())
}
