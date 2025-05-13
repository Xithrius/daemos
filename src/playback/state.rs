use std::{
    fs::File,
    io::BufReader,
    time::{Duration, Instant},
};

use color_eyre::{Result, eyre::bail};
use crossbeam::{
    channel::{Receiver, Sender},
    utils::Backoff,
};
use rodio::{Decoder, OutputStream, Sink};
use serde::{Deserialize, Serialize};
use tracing::{debug, error, info};

use crate::database::models::tracks::Track;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum PlayerEvent {
    TrackChanged(Track),
    TrackProgress(Duration),
    CurrentVolume(f32),
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum PlayerCommand {
    Create(Track),
    Play,
    Pause,
    Resume,
    SkipNext,
    SkipPrevious,
    Volume,
    SetVolume(f32),
    Position,
    SetPosition(Duration),
}

pub struct Player {
    #[allow(dead_code)]
    stream: OutputStream,
    sink: Sink,

    player_event_tx: Sender<PlayerEvent>,
    player_cmd_rx: Receiver<PlayerCommand>,

    #[allow(dead_code)]
    track_queue: Vec<Track>,
}

impl Player {
    pub fn new(
        player_event_tx: Sender<PlayerEvent>,
        player_cmd_rx: Receiver<PlayerCommand>,
    ) -> Result<Self> {
        let backoff = Backoff::new();
        let timeout = Instant::now() + Duration::from_secs(10);

        let (stream, sink) = loop {
            match OutputStream::try_default() {
                Ok((stream, handle)) => match Sink::try_new(&handle) {
                    Ok(sink) => break (stream, sink),
                    Err(err) => {
                        error!("Sink creation failed: {}", err);
                    }
                },
                Err(err) => {
                    error!("Audio device not available: {}", err);
                }
            }

            if Instant::now() > timeout {
                bail!("Timed out waiting for audio device");
            }

            backoff.snooze();
        };

        info!("Audio device found!");

        Ok(Self {
            stream,
            sink,
            player_event_tx,
            player_cmd_rx,
            track_queue: Vec::new(),
        })
    }

    pub fn create(&self) {
        while let Ok(command) = self.player_cmd_rx.recv() {
            if let Err(err) = self.handle_command(&command) {
                error!(
                    "Processing player command {:?} failed with error {}",
                    command, err
                );
            }
        }
    }

    fn create_player_track(&self, track: &Track) -> Result<()> {
        if !self.sink.empty() {
            self.sink.clear();
        }

        let track_file_path = track.path.clone();

        debug!("Appended file {:?} to sink, and playing", track.path);

        let file = File::open(track_file_path)?;
        let decoder = Decoder::new(BufReader::new(file))?;

        self.sink.append(decoder);
        self.sink.set_volume(0.5);
        self.sink.play();

        Ok(())
    }

    fn handle_command(&self, command: &PlayerCommand) -> Result<()> {
        debug!("Player received command: {:?}", command);

        match command {
            PlayerCommand::Create(track) => {
                self.create_player_track(track)?;

                Ok(())
            }
            PlayerCommand::Play => {
                self.sink.play();

                Ok(())
            }
            PlayerCommand::Pause => {
                self.sink.pause();

                Ok(())
            }
            PlayerCommand::Resume => {
                if self.sink.is_paused() {
                    self.sink.play();
                } else {
                    debug!("No track to resume");
                }

                Ok(())
            }
            PlayerCommand::SkipNext => {
                self.sink.skip_one();

                Ok(())
            }
            PlayerCommand::SkipPrevious => {
                todo!();
            }
            PlayerCommand::Volume => {
                let volume = self.sink.volume();

                let _ = self
                    .player_event_tx
                    .send(PlayerEvent::CurrentVolume(volume));

                Ok(())
            }
            PlayerCommand::SetVolume(volume_value) => {
                self.sink.set_volume(*volume_value);

                Ok(())
            }
            PlayerCommand::Position => {
                let position = self.sink.get_pos();

                let _ = self
                    .player_event_tx
                    .send(PlayerEvent::TrackProgress(position));

                Ok(())
            }
            PlayerCommand::SetPosition(duration) => {
                if let Err(err) = self.sink.try_seek(*duration) {
                    bail!("Failed to set duration: {:?}", err);
                };

                let position = self.sink.get_pos();

                let _ = self
                    .player_event_tx
                    .send(PlayerEvent::TrackProgress(position));

                Ok(())
            }
        }
    }
}
