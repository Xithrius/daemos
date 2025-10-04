use std::io::{Read, Seek};

use parking_lot::Mutex;
use reqwest::blocking::Response;
use symphonia::core::io::MediaSource;

pub struct HttpSource {
    pub inner: Mutex<Response>,
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
