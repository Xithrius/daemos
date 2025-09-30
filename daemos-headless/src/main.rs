use std::{
    collections::VecDeque,
    io::{Read, Seek},
    sync::mpsc::{Receiver, SyncSender, TryRecvError, sync_channel},
    thread,
    time::Duration,
};

use color_eyre::Result;
use parking_lot::Mutex;
use reqwest::blocking::Response;
use rodio::{OutputStream, Sink};
use symphonia::{
    core::{
        audio::SampleBuffer,
        codecs::{Decoder, DecoderOptions},
        formats::{FormatOptions, FormatReader},
        io::{MediaSource, MediaSourceStream},
        meta::MetadataOptions,
    },
    default::{get_codecs, get_probe},
};

#[derive(Debug)]
struct PcmChunk {
    channels: u16,
    sample_rate: u32,
    samples: Vec<f32>,
}

struct PcmSource {
    rx: Receiver<PcmChunk>,
    buffer: VecDeque<f32>,
    channels: u16,
    sample_rate: u32,
}

impl Iterator for PcmSource {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(s) = self.buffer.pop_front() {
            return Some(s);
        }
        // Block for at least one chunk when empty, then drain any immediately available
        let first = match self.rx.recv() {
            Ok(c) => c,
            Err(_) => return None,
        };
        debug_assert_eq!(first.channels, self.channels);
        debug_assert_eq!(first.sample_rate, self.sample_rate);
        self.buffer = first.samples.into();
        loop {
            match self.rx.try_recv() {
                Ok(mut more) => {
                    debug_assert_eq!(more.channels, self.channels);
                    debug_assert_eq!(more.sample_rate, self.sample_rate);
                    if self.buffer.is_empty() {
                        self.buffer = more.samples.into();
                    } else {
                        self.buffer.extend(more.samples.drain(..));
                    }
                }
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => break,
            }
        }
        self.buffer.pop_front()
    }
}

impl rodio::Source for PcmSource {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        self.channels
    }
    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }
    fn total_duration(&self) -> Option<std::time::Duration> {
        None
    }
}

struct HttpSource {
    inner: Mutex<Response>,
}

impl Read for HttpSource {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let mut guard = self.inner.lock();
        guard.read(buf)
    }
}

impl MediaSource for HttpSource {
    fn is_seekable(&self) -> bool {
        false
    }
    fn byte_len(&self) -> Option<u64> {
        None
    }
}

impl Seek for HttpSource {
    fn seek(&mut self, _pos: std::io::SeekFrom) -> std::io::Result<u64> {
        Err(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "seek not supported",
        ))
    }
}

fn main() -> Result<()> {
    // --- HTTP stream setup ---
    println!("http: starting request to /track");
    let client = reqwest::blocking::ClientBuilder::new()
        .timeout(None)
        .build()?;
    let resp = client
        .get("http://127.0.0.1:7070/track")
        .header(reqwest::header::ACCEPT, "*/*")
        .send()?
        .error_for_status()?;
    println!("http: connected, status {}", resp.status());

    // --- Setup Symphonia decoder with streaming source ---
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
    println!("probe: format detected");

    let format = probed.format;
    let track = format.default_track().unwrap();
    println!("track: codec={:?}", track.codec_params.codec);

    let decoder = get_codecs().make(&track.codec_params, &DecoderOptions { verify: false })?;
    println!("decoder: initialized (verify=false)");

    // --- Setup Rodio output ---
    let (_stream, handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&handle)?;

    // Bounded producer-consumer: decoder thread -> sync_channel -> sink
    let (tx, rx) = sync_channel::<PcmChunk>(64);
    println!("pipeline: channel capacity=64");

    println!("producer: thread spawn");
    thread::spawn(move || {
        // Move format and decoder into producer thread
        let _ = decode_producer(format, decoder, tx);
    });

    // Initialize a blocking Source from the first chunk to set format, then stream on demand
    let first = rx.recv().expect("producer ended before first chunk");
    println!(
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
    println!("main: playback finished");
    Ok(())
}

fn decode_producer(
    mut format: Box<dyn FormatReader>,
    mut decoder: Box<dyn Decoder>,
    tx: SyncSender<PcmChunk>,
) -> Result<()> {
    println!("producer: started");
    let mut produced = 0usize;
    // Target chunking: larger chunks to reduce refill boundaries (~120-170ms @48kHz)
    const TARGET_FRAMES_PER_CHUNK: usize = 8192;
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
            if produced % 30 == 0 {
                println!(
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
    println!("producer: finished (eof or consumer dropped)");
    Ok(())
}
