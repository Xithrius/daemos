# TODO

## Now

- [x] UI sections (fake/real data)
    - [x] Music
    - [x] Playlist tree

- [-] Basic UI
    - [x] List of all available music in folder items
        - [x] All available music
    - [ ] Playback
        - [ ] Play
        - [ ] Pause
        - [ ] Next track
        - [ ] Previous track

- [ ] Saving track/playlist data to disk
    - [ ] Sqlite connection
    - [ ] Playlist tree table
        - One table, each row has a parent ID
        - If a row doesn't have a parent, that means it's the root
        - Create some query that would create a vector of vectors for the playlist tree UI
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

## Later

- Shuffle
    - Pseudo Random (don't play the same thing twice until everything is played)
    - True Random (Pick any other track)
