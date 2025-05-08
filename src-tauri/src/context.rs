use parking_lot::Mutex;

#[derive(Default)]
pub struct ContextInner {
    counter: u32,
}

pub type Context = Mutex<ContextInner>;
