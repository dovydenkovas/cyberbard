use crate::stream::Stream;
use crate::source::Source;

pub trait Audio {
    fn get_source(&self) -> Result<Box<dyn Source>, AudioError>;
    fn set_source(&mut self, source: Box<dyn Source>);
    fn get_volume(&self) -> u8;
    fn set_volume(&mut self, volume: u8);
    fn is_looped(&self) -> bool;
    fn looped(&mut self, looped: bool);
    fn get_stream(&self) -> Option<Stream>;
    fn insert_audio(&mut self, index: usize, audio: Box<dyn Audio>) -> Result<(), AudioError>;
    fn erase_audio(&mut self, index: usize) -> Result<(), AudioError>;
    fn get_audio(&self, index: usize) -> Result<Box<dyn Audio>, AudioError>;
    fn audio_count(&self) -> usize;
    fn clone_box(&self) -> Box<dyn Audio>;
}

impl Clone for Box<dyn Audio> {
    fn clone(&self) -> Box<dyn Audio> {
        self.clone_box()
    }
}

#[derive(Debug, PartialEq)]
pub enum AudioError {
    NotAComposition,
    NotATrack,
    OutOfRange,
}
