#[macro_export]
macro_rules! vertical_separator {
    ($ui:expr) => {
        $ui.add(Separator::default().vertical())
    };
}

#[macro_export]
macro_rules! horizontal_separator {
    ($ui:expr) => {
        $ui.add(Separator::default().horizontal())
    };
}
