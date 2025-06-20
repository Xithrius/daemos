# TODO

## Now

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
  - [x] Show currently selected track title

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

- [ ] Searching

  - [ ] Tracks

    - [ ] Contains lowercase version
    - [ ] Fuzzy searching
    - [ ] By attributes
      - [ ] First, get search by track name working
      - [ ] Attribute dropdown
      - [ ] Combine the two to have track-attribute searching

  - [ ] Playlists

- [ ] Sorting

  - [ ] Columns
  - [ ] Track name sorting
    - [ ] By full name
    - [ ] By regex group

- [ ] Autoplay

  - [ ] Iterative
    - [x] Play next track in the table once the current one is done
    - [x] Once last track is done, go to the start
    - [ ] Reverse button on playback bar\*
  - [ ] Shuffle
    - [x] Pseudo Random (don't play the same thing twice until everything is played)
    - [x] True Random (Pick any other track)
    - [ ] Similar tracks (Hash distance?)
    - [ ] [Fisher–Yates shuffle](https://en.wikipedia.org/wiki/Fisher%E2%80%93Yates_shuffle)
  - [ ] When shuffle is selected, going back a track should go to the one previously played

- [ ] Fonts

  - [x] Change font to Space mono font
  - [ ] font size\*

\* Should be configurable

## Issues

- [ ] If a new track is skipped on insert due to being a duplicate, the spinner will spin forever
- [ ] Filtered tracks in the track table should be a vector of usizes that point to an index in the original tracks vector
- [ ] Keep track of previously played tracks such that they can be seen in a recently played tab, and also go back to them in playback controls
- [ ] Hitting playback controls (forward/backward) doesn't contribute to seen list for pseudo random shuffle (make this configurable)
