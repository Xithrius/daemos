use std::io::{Read, Seek};

use color_eyre::Result;
use parking_lot::Mutex;
use reqwest::blocking::Response;
use symphonia::core::io::MediaSource;
use tracing::info;

pub struct HttpSource {
    pub inner: Mutex<Response>,
}

impl HttpSource {
    pub fn new(inner: Response) -> Self {
        Self {
            inner: Mutex::new(inner),
        }
    }
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

pub fn request_http_stream(track_url: &str) -> Result<HttpSource> {
    let client = reqwest::blocking::ClientBuilder::new()
        .timeout(None)
        .build()?;
    let resp = client
        .get(track_url)
        .header(reqwest::header::ACCEPT, "*/*")
        .send()?
        .error_for_status()?;
    info!("http: connected, status {}", resp.status());

    let source = HttpSource::new(resp);

    Ok(source)
}
