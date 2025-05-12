use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

use blake3::{Hash, Hasher};
use color_eyre::Result;

pub fn hash_file<P: AsRef<Path>>(path: P) -> Result<Hash> {
    let file = File::open(path)?;

    let mut reader = BufReader::new(file);
    let mut hasher = Hasher::new();

    let mut buffer = [0u8; 8192]; // 8KB buffer

    loop {
        let n = reader.read(&mut buffer)?;
        if n == 0 {
            break;
        }

        hasher.update(&buffer[..n]);
    }

    Ok(hasher.finalize())
}
