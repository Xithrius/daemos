use std::path::PathBuf;

use actix_files::NamedFile;
use actix_web::{HttpRequest, HttpResponse, Result, http::header, web};
use tracing::info;

const TRACK_PATH: &str = "sample_track.flac";

#[allow(clippy::future_not_send)]
async fn stream_file(req: HttpRequest) -> Result<HttpResponse> {
    info!("Streaming file");

    let path: PathBuf = TRACK_PATH.into();
    let file = NamedFile::open(path)?;

    let mut resp = file.into_response(&req);
    resp.headers_mut().insert(
        header::ACCEPT_RANGES,
        header::HeaderValue::from_static("bytes"),
    );
    Ok(resp)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/track", web::get().to(stream_file));
}
