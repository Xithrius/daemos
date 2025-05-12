use std::{
    fs::File,
    io::BufReader,
    time::{Duration, Instant},
};

use color_eyre::{Result, eyre::bail};
use crossbeam::{channel::Receiver, utils::Backoff};
use rodio::{Decoder, OutputStream, Sink};
use serde::{Deserialize, Serialize};
use tracing::{debug, error, info};

use crate::database::models::tracks::Track;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum PlayerCommand {
    Create(Track),
    Play,
    Pause,
    Resume,
    SkipNext,
    SkipPrevious,
    SetVolume(f32),
}

pub struct Player {
    #[allow(dead_code)]
    stream: OutputStream,
    sink: Sink,
    rx: Receiver<PlayerCommand>,

    // TODO: Mutex the queue between this and the main thread
    #[allow(dead_code)]
    track_queue: Vec<Track>,
}

impl Player {
    pub fn new(rx: Receiver<PlayerCommand>) -> Result<Self> {
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
            rx,
            track_queue: Vec::new(),
        })
    }

    pub fn create(&self) {
        while let Ok(command) = self.rx.recv() {
            if let Err(err) = self.handle_command(&command) {
                error!(
                    "Processing player command {:?} failed with error {}",
                    command, err
                );
            }
        }
    }

    #[allow(dead_code)]
    fn create_player_track(&self) {
        todo!()
    }

    fn handle_command(&self, command: &PlayerCommand) -> Result<()> {
        debug!("Player received command: {:?}", command);

        match command {
            PlayerCommand::Create(track) => {
                let track_file_path = track.path.clone();

                let file = File::open(track_file_path)?;
                let decoder = Decoder::new(BufReader::new(file))?;

                self.sink.append(decoder);
                self.sink.set_volume(0.5);
                self.sink.play();

                debug!("Appended file to sink, and playing");

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
            PlayerCommand::SetVolume(volume_value) => {
                self.sink.set_volume(*volume_value);

                Ok(())
            }
        }
    }
}
