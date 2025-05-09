use iced::{
    Color, Element, Event,
    Length::{self, Shrink},
    Subscription, Task, Theme,
    event::listen_with,
    keyboard::{self, Key, key::Named},
    widget::{button, container, pick_list, row},
};

use crate::files::open::{get_audio_files, select_folders_dialog};

#[derive(Default)]
pub struct Application {
    debug: bool,
    theme: Theme,
}

#[derive(Debug, Clone)]
pub enum Message {
    ThemeChanged(Theme),
    OpenFolder,
    DebugToggled,
}

impl Application {
    pub fn new() -> (Self, Task<Message>) {
        (
            Self {
                debug: false,
                theme: Theme::Dark,
            },
            Task::none(),
        )
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::ThemeChanged(theme) => {
                self.theme = theme;
            }
            Message::OpenFolder => {
                let selected_folders = select_folders_dialog();
                println!("{:?}", selected_folders);

                if let Some(selected_folders) = selected_folders {
                    let mut audio_files = Vec::new();

                    for selected_folder in selected_folders {
                        let files = get_audio_files(selected_folder);
                        audio_files.extend(files);
                    }

                    println!("{:?}", audio_files);
                }
            }
            Message::DebugToggled => {
                self.debug = !self.debug;
            }
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        listen_with(|event, _, _| {
            if let Event::Keyboard(keyboard::Event::KeyPressed {
                key: Key::Named(named),
                ..
            }) = event
            {
                return match named {
                    Named::F1 => Some(Message::OpenFolder),
                    Named::F3 => Some(Message::DebugToggled),
                    _ => None,
                };
            }

            None
        })
    }

    pub fn view(&self) -> Element<Message> {
        let choose_theme = pick_list(Theme::ALL, Some(&self.theme), Message::ThemeChanged)
            .width(Shrink)
            .width(200);

        let open_folder_button = button("Open folder")
            .padding(10)
            .on_press(Message::OpenFolder);

        let content: Element<_> = row![choose_theme, open_folder_button]
            .spacing(20)
            .padding(20)
            .into();

        container(if self.debug {
            content.explain(Color::BLACK)
        } else {
            content
        })
        .align_top(Length::Fill)
        .align_right(Length::Fill)
        .into()
    }

    pub fn theme(&self) -> Theme {
        self.theme.clone()
    }
}
