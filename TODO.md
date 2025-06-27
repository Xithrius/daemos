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

    - [ ] By attributes
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
    - [ ] Reverse button on playback bar\*
  - [ ] Shuffle
    - [ ] Similar tracks (Hash distance?)
    - [ ] [Fisher–Yates shuffle](https://en.wikipedia.org/wiki/Fisher%E2%80%93Yates_shuffle)
  - [ ] When shuffle is selected, going back a track should go to the one previously played

## Issues

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

- Implement MPRIS via [zbus](https://docs.rs/crate/zbus/5.7.1)/[zbus_macros](https://docs.rs/crate/zbus_macros/5.7.1) to allow operations with playerctl

- [ ] font size\*

- [ ] Crossfade\*

\* Should be configurable
