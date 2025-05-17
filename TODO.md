# TODO

## Now

- [x] UI sections (fake/real data)
    - [x] Music
    - [x] Playlist tree

- [x] Basic UI
    - [x] List of all available music in folder items
        - [x] All available music
    - [x] Playback
        - [x] Play
        - [x] Pause
        - [x] Next track
        - [x] Previous track

- [ ] Tracks and playlists
    - [ ] Sqlite database
        - [x] Connection
        - [x] Table definitions and creation
            - [x] Tracks
            - [x] Playlists
        - [x] CRUD for each table
            - [x] Tracks
            - [ ] Playlists
    - [ ] Tracks
        - [x] Should not query once per frame
            - [x] At startup query once then put into vector
        - [x] Hashing
            - [x] On track insert, hash said track
            - [ ] Perhaps have a tab for tracking duplicates
        - [ ] Querying/inserting should be done on another thread
        - [ ] Validation of tracks (via settings)
            - [ ] At startup
            - [ ] Every once in a while
            - [ ] __DO NOT__ remove file, instead mark with warning and ask user to either correct path or remove entry
    - [ ] Playlist tree table
        - One table, each row has a parent ID
        - If a row doesn't have a parent, that means it's the root
        - Create some query that would create a vector of vectors for the playlist tree UI

- [ ] Track UI table
    - [ ] Sort by different columns
    - [ ] Track name sorting
        - [ ] By full name
        - [ ] By regex group

- [ ] Playback
    - [ ] Control
        - [ ] Playback
            - [x] Play
            - [x] Pause
            - [x] Toggle
            - [ ] Next track
            - [ ] Previous track
        - [x] Volume bar
            - [x] Click to set volume
            - [x] Drag to set volume
        - [ ] Seek bar
            - [ ] See live elapsed and total duration updates
            - [ ] Click to set time
            - [ ] Drag to set time
    - [ ] Visualization
        - [ ] Seek bar elapsed and total track duration

- [ ] Settings
    - [x] Popup widget (another window)
    - [ ] Default volume on startup
    - [ ] Default ordering of tracks

    - [ ] Default folder to open when adding tracks
    - [ ] Toggle to hash tracks on insert
    - [ ] (re)calculation of all track hashes

- [ ] First time setup
    - [ ] Offline stats to local storage
        - [ ] Times a track has been played

## Later

- Shuffle
    - Pseudo Random (don't play the same thing twice until everything is played)
    - True Random (Pick any other track)
    - Similar tracks (Hash distance?)

## Issues

Centering playback bar controls: https://github.com/emilk/egui/discussions/1197

- [ ] Playback seek bar skips a second, or pauses on a duration for too long
- [ ] Vertically center the playback previous/toggle/next controls
