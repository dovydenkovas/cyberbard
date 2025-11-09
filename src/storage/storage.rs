use super::source::Source;

/// Storage trait describe interface to audio sources manipulation.
pub trait Storage {
    fn load_sources(&mut self);
    fn get(&self, index: usize) -> Box<dyn Source>;
    fn size(&self) -> usize;
    fn attach_tag(&mut self, index: usize, title: String);
    fn unattach_tag(&mut self, index: usize, title: String);
    fn find_by_tag(&self, substr: String) -> Vec<usize>;
    fn find_by_title(&self, substr: String) -> Vec<usize>;
}
