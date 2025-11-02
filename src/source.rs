use crate::stream::Stream;


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
