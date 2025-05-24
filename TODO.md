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
    - [ ] Validation of tracks (via settings)
      - [ ] At startup
      - [ ] Every once in a while
      - [ ] **DO NOT** remove file, instead mark with warning and ask user to either correct path or remove entry
  - [ ] Playlist tree table
    - One table, each row has a parent ID
    - If a row doesn't have a parent, that means it's the root
    - Create some query that would create a vector of vectors for the playlist tree UI

- [ ] Track UI table

  - [ ] Sort by different columns
  - [ ] Track name sorting
    - [ ] By full name
    - [ ] By regex group
  - [ ] Search by attribute
    - [ ] First, get search by track name working
    - [ ] Attribute dropdown
    - [ ] Combine the two to have track-attribute searching

- [ ] Playback

  - [ ] Control
    - [ ] Playback
      - [x] Play
      - [x] Pause
      - [x] Toggle
      - [ ] Next track
      - [ ] Previous track
      - [ ] Go back 10 seconds\*
      - [ ] Go forward 10 seconds\*
    - [x] Volume bar
      - [x] Click to set volume
      - [x] Drag to set volume
    - [x] Seek bar
      - [x] See live elapsed and total duration updates
      - [x] Click to set time
      - [x] Drag to set time
  - [x] Visualization
    - [x] Seek bar elapsed and total track duration
  - [ ] Show currently selected track title

- [x] Database

  - [x] Move to another thread
  - [x] Create an unbounded channel for receiving on thread's end, and sending from UI
  - [x] Create another unbounded channel to receive in UI, and send from thread
  - [x] Create commands to communicate to said thread with
  - [x] Replace all current command calls with sending messages

- [ ] Tabs

  - [ ] Tracks
  - [ ] Playlists
  - [ ] Database tasks (other tasks in the future?)

- [ ] Keybinds

  - [ ] Ctrl+Shift+O to open a folder of tracks in the OS file explorer
  - [ ] Space for toggling pause/play
  - [ ] F3 to toggle the debug wireframe
  - [ ] Ctrl+F to focus the search input box

- [ ] Settings

  - [x] Popup widget (another window)
  - [ ] Default volume on startup
  - [ ] Default ordering of tracks
  - [ ] General config settings

  - [ ] Default folder to open when adding tracks
  - [ ] Toggle to hash tracks on insert
  - [ ] (re)calculation of all track hashes

- [ ] First time setup

  - [ ] Offline stats to local storage
    - [ ] Times a track has been played

- [ ] General UI look/feel

  - [x] Change font to Space mono font
  - [ ] font size\*

\* Should be configurable

## Later

- Shuffle
  - Pseudo Random (don't play the same thing twice until everything is played)
  - True Random (Pick any other track)
  - Similar tracks (Hash distance?)

## Issues

- [x] Playback seek bar skips a second, or pauses on a duration for too long
- [x] Mutability of specific component attributes is getting messy, either create an inner context or get a better method of passing things around
- [ ] Vertically center the playback previous/toggle/next controls
  - [Possibly relevant issue](https://github.com/emilk/egui/discussions/1197)
- [ ] Playback timer has a couple frames of idle state then playing when autoplay selects a new track
