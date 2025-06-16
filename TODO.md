# TODO

## Now

- [x] UI sections (fake/real data)

  - [x] Music
  - [x] Playlists

- [x] Basic UI

  - [x] List of all available music in folder items
    - [x] All available music
  - [x] Playback
    - [x] Play
    - [x] Pause
    - [x] Next track
    - [x] Previous track

- [ ] Database

  - [x] Sqlite connection
  - [x] Table definitions
    - [x] Tracks
    - [x] Playlists
    - [x] Tags
  - [ ] CRUD
    - [x] Tracks
    - [x] Playlists
    - [ ] Tags

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
      - [x] Next track
      - [x] Previous track
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

- [x] Database background jobs

  - [x] Move to another thread
  - [x] Create an unbounded channel for receiving on thread's end, and sending from UI
  - [x] Create another unbounded channel to receive in UI, and send from thread
  - [x] Create commands to communicate to said thread with
  - [x] Replace all current command calls with sending messages

- [x] Tabs

  - [x] Install through [egui_dock](https://crates.io/crates/egui_dock)
  - [x] Left panel
    - [x] Playlists
  - [x] Right panel
    - [x] Tracks
    - [x] Tags
    - [x] Database tasks

- [x] Keybinds

  - [x] `Ctrl+Shift+O` to open a folder of tracks in the OS file explorer
  - [x] `Space` for toggling pause/play
  - [x] `F3` to toggle the debug wireframe
  - [x] `Ctrl+F` to focus the search input box
  - [x] `Ctrl+,` to toggle settings popup window

- [ ] Settings

  - [x] Popup widget (another window)
  - [ ] Default volume on startup
  - [ ] Default ordering of tracks
  - [ ] General config settings
  - [ ] Default folder to open when adding tracks
  - [ ] Toggle to hash tracks on insert
  - [ ] (re)calculation of all track hashes
  - [ ] Validation of tracks
    - [ ] Timing
      - [ ] At startup
      - [ ] Every once in a while
      - [ ] Check type matching
        - [ ] Path
        - [ ] Name
        - [ ] Hash recalculation
      - [ ] If invalid, mark with warning and ask user to either correct/remove entry

- [ ] First time setup

  - [ ] Offline stats to local storage
    - [ ] Times a track has been played

- [ ] Misc

  - [x] When executing tasks, have a loading circle at the top right of the screen\*
    - [x] Number when tasks start executing
    - [x] Change number when tasks are completed
  - [x] [Catppuccin themes](https://crates.io/crates/catppuccin-egui)
  - [ ] Fuzzy track searching

- [ ] Autoplay

  - [ ] Iterative
    - [x] Play next track in the table once the current one is done
    - [x] Once last track is done, go to the start
    - [ ] Reverse button on playback bar\*
  - [ ] Shuffle
    - [x] Pseudo Random (don't play the same thing twice until everything is played)
    - [x] True Random (Pick any other track)
    - [ ] Similar tracks (Hash distance?)
  - [ ] When shuffle is selected, going back a track should go to the one previously played

- [ ] Fonts

  - [x] Change font to Space mono font
  - [ ] font size\*

\* Should be configurable

## Issues

- [x] Playback seek bar skips a second, or pauses on a duration for too long
- [x] Mutability of specific component attributes is getting messy, either create an inner context or get a better method of passing things around
- [x] If a playlist is playing a track and we focus elsewhere (such as another playlist), that playlist should still autoplay in the correct order
- [x] Volume is reset when a new track is selected with autoplay
- [x] When selecting a new track to play on another playlist, autoplay and now playing track labels in the playback bar aren't visually set at the same time
- [x] Playback timer has a couple frames of idle state then playing when autoplay selects a new track
- [x] Horizontally center playback seek bar
- [ ] If a new track is skipped on insert due to being a duplicate, the spinner will spin forever
- [ ] Filtered tracks in the track table should be a vector of usizes that point to an index in the original tracks vector
- [ ] Move attributes of UI components to higher-up context, as some components would have to directly call other components instead of calling context methods
  - [ ] `TrackState` is duplicated between the playback bar and the track table, combine into one structure and keep in playback context
  - [ ] Move any playlist/track(s) attributes into playback context
- [ ] Keep track of previously played tracks such that they can be seen in a recently played tab, and also go back to them in playback controls
