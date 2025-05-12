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

- [ ] Saving track/playlist data to disk
    - [ ] Sqlite database
        - [ ] Connection
        - [ ] Table definitions and creation
            - [ ] Tracks
            - [ ] Playlists
        - [ ] CRUD for each table
            - [ ] Tracks
            - [ ] Playlists
    - [ ] Tracks
        - [ ] Each row has an ID
        - [ ] Hashing
            - [ ] At first startup and as a setting, hash the track
            - [ ] Perhaps have a tab for tracking duplicates
            - [ ] Have a setting to (re)calculate all hashes
        - [ ] Of course, have the file path to the track
        - [ ] Validation of tracks (via settings)
            - [ ] At startup
            - [ ] Every once in a while
            - [ ] __DO NOT__ remove file, instead mark with warning and ask user to either correct path or remove entry
    - [ ] Playlist tree table
        - One table, each row has a parent ID
        - If a row doesn't have a parent, that means it's the root
        - Create some query that would create a vector of vectors for the playlist tree UI

- [ ] Playback
    - [ ] Control
        - [ ] Playback
            - [ ]
        - [ ] Seek bar
            - [ ] Click to set time
            - [ ] Drag to set time
    - [ ] Visualization
        - [ ] Seek bar elapsed and total track duration

## Later

- Shuffle
    - Pseudo Random (don't play the same thing twice until everything is played)
    - True Random (Pick any other track)
    - Similar tracks (Hash distance?)

## Issues

Centering playback bar controls: https://github.com/emilk/egui/discussions/1197
