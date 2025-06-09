pub mod create_playlist;

pub trait UIModal {
    fn set_visibility(&mut self, visibility: bool);
}
