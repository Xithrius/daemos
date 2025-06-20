use std::collections::HashSet;

use uuid::Uuid;

use crate::database::models::{playlists::playlist::Playlist, tracks::Track};

#[derive(Debug, Clone, Default)]
pub struct LoadedTracksContext {
    loaded: Vec<Track>,
    ids: HashSet<Uuid>,

    // TODO: Convert to vector of indices
    filtered: Vec<Track>,
}

#[derive(Debug, Clone, Default)]
pub struct LoadedPlaylistsContext {
    loaded: Vec<Playlist>,
    ids: HashSet<Uuid>,
}

#[derive(Debug, Clone, Default)]
pub struct LoadedContext {
    tracks: LoadedTracksContext,
    playlists: LoadedPlaylistsContext,
}
