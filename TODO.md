# TODO

## Now

- [ ] Playback

  - [ ] Control

    - [ ] Playback

      - [ ] Go back 10 seconds\*
      - [ ] Go forward 10 seconds\*

- [ ] Settings

  - [ ] Write to config path
  - [ ] Default volume on startup
  - [ ] Default ordering of tracks
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

- [ ] Filtered tracks in the track table should be a vector of usizes that point to an index in the original tracks vector
- [ ] Keep track of previously played tracks such that they can be seen in a recently played tab, and also go back to them in playback controls
- [ ] Hitting playback controls (forward/backward) doesn't contribute to seen list for pseudo random shuffle (make this configurable)

## In the future

- [ ] Tags

  - [ ] Database CRUD
  - [ ] Groups
  - [ ] Adding to tracks/playlists

- [ ] First time setup

  - [ ] Offline stats to local storage
    - [ ] Times a track has been played

- [ ] [Notifications](https://github.com/ItsEthra/egui-notify)
  - [ ] User actions (creating playlists)
  - [ ] Background task completions (finished inserting tracks)

## Useful links

- [egui demo](https://www.egui.rs/#demo)
- [3rd party egui crates](https://github.com/emilk/egui/wiki/3rd-party-egui-crates)
