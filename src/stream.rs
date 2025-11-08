/// Audio stream. Low level structure to send audio to player.
use rodio::Sink;

pub struct Stream {
    sinks: Vec<Sink>
}

impl Stream {
    pub fn new() -> Stream {
        Stream { sinks: vec![]  }
    }

    pub fn from_synk(sink: Sink) -> Stream {
        Stream {sinks: vec![sink]}
    }

    pub fn is_empty(&self) -> bool {
        self.sinks.len() == 0
    }

    pub fn get_sinks(self) -> Vec<Sink> {
        self.sinks
    }

    pub fn merge(&mut self, other: Stream) {
       let sinks = other.get_sinks();
       for sink in sinks {
           self.sinks.push(sink);
       }
    }


}
