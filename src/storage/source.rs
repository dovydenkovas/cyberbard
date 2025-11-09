use super::stream::Stream;

/// Source trait describe interface of music in storage.
/// Provides audio Stream and title.
pub trait Source {
    fn get_stream(&self) -> Stream;
    fn get_title(&self) -> String;
    fn clone_box(&self) -> Box<dyn Source>;
}

impl Clone for Box<dyn Source> {
    fn clone(&self) -> Box<dyn Source> {
        self.clone_box()
    }
}
