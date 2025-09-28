pub mod create_playlist;

pub trait UIModal {
    fn visibility(&self) -> bool;
    fn set_visibility(&mut self, visibility: bool);
}
