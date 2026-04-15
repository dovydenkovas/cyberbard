//   Cyberbard music player for board role-playing games.
//   Copyright (C) 2025  Aleksandr Dovydenkov <asd@altlinux.org>
//
//   This program is free software: you can redistribute it and/or modify
//   it under the terms of the GNU General Public License as published by
//   the Free Software Foundation, either version 3 of the License, or
//   (at your option) any later version.
//
//   This program is distributed in the hope that it will be useful,
//   but WITHOUT ANY WARRANTY; without even the implied warranty of
//   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//   GNU General Public License for more details.
//
//   You should have received a copy of the GNU General Public License
//   along with this program.  If not, see <https://www.gnu.org/licenses/>

mod threadstream;
mod trackstream;

pub trait Opener {
    fn source(&mut self) -> Result<Box<dyn rodio::Source + Send>, Box<dyn std::error::Error>>;
    fn total_duration(&self) -> f32;
}

use crate::stream::trackstream::TrackStream;
use std::fmt;
use std::sync::Mutex;
use std::time::Duration;
use threadstream::ThreadStream;

lazy_static::lazy_static! {
static ref OSTREAM: Mutex<rodio::OutputStream> =
    Mutex::new(rodio::OutputStreamBuilder::open_default_stream().unwrap());
}

pub struct Stream {
    threads: Vec<ThreadStream>,
    total_volume: f32,
}

impl Stream {
    pub fn new(threads: Vec<ThreadStream>, total_volume: f32) -> Stream {
        Stream {
            threads,
            total_volume,
        }
    }

    pub fn from_source(
        src: Box<dyn Opener + Send>,
        volume: f32,
    ) -> Result<Stream, Box<dyn std::error::Error>> {
        let mut lock = OSTREAM
            .lock()
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
        let pl = ThreadStream::new(&mut lock, vec![TrackStream::new(src, volume)], 1.0)
            .ok_or(StreamError {})?;
        Ok(Stream {
            threads: vec![pl],
            total_volume: 0.0,
        })
    }

    pub fn set_total_volume(&mut self, volume: f32) {
        let volume = volume.clamp(0.0, 1.0);
        self.total_volume = volume;
        for pl in self.threads.iter_mut() {
            pl.update_volume(volume);
        }
    }

    pub fn set_partial_volume(&mut self, volume: f32, thread_index: usize, audio_index: usize) {
        if !self.threads.is_empty() {
            self.threads[thread_index].set_partial_volume(volume, audio_index);
        }
    }

    pub fn is_empty(&self) -> bool {
        self.threads.len() == 0
    }

    pub fn get_threads(self) -> Vec<ThreadStream> {
        self.threads
    }

    pub fn get_current_playing(&self) -> Vec<usize> {
        let mut res = vec![];
        for th in &self.threads {
            res.push(th.current);
        }
        res
    }

    pub fn merge(&mut self, other: Stream) {
        for (i, pl) in other.threads.into_iter().enumerate() {
            if i < self.threads.len() {
                self.threads[i].extend(pl);
            } else {
                self.threads.extend(vec![pl]);
            }
        }
    }

    pub fn sync(&mut self, new: Stream) {
        self.total_volume = new.total_volume;
        for (i, pl) in new.threads.into_iter().enumerate() {
            if i < self.threads.len() {
                self.threads[i].replace_sources(pl.tracks);
                self.threads[i].update_volume(self.total_volume);
            } else {
                self.threads.extend(vec![pl]);
                self.threads[i].update_volume(self.total_volume);
            }
        }
    }

    pub fn merge_parallel(&mut self, other: Stream) {
        self.threads.extend(other.threads);
    }

    pub fn play(&mut self) {
        for thread in self.threads.iter_mut() {
            thread.play();
        }
    }

    pub fn pause(&mut self) {
        for thread in self.threads.iter_mut() {
            thread.pause();
        }
    }

    pub fn stop(&mut self) {
        for thread in self.threads.iter_mut() {
            thread.stop();
        }
    }

    /// Index and progress of current track in each thread.
    pub fn get_position(&self) -> Vec<(usize, f32)> {
        let mut positions = vec![];
        for thread in self.threads.iter().as_ref() {
            positions.push(thread.get_position())
        }
        positions
    }

    pub fn update(&mut self) {
        for thread in self.threads.iter_mut() {
            thread.update();
        }
        std::thread::sleep(Duration::from_millis(5));
    }

    pub fn goto_track(&mut self, thread: usize, track: usize) {
        if !self.threads.is_empty() {
            self.threads[thread].goto(track);
        }
    }
}

#[derive(Debug)]
struct StreamError {}

impl fmt::Display for StreamError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Stream error")
    }
}

impl std::error::Error for StreamError {}

#[cfg(test)]
mod tests {
    use crate::storage::localstorage::LocalOpener;

    use super::*;

    #[test]
    fn stream_empty() {
        let mut stream = Stream::new(vec![], 1.0);

        assert_eq!(1.0, stream.total_volume);
        stream.set_total_volume(0.6);
        assert_eq!(0.6, stream.total_volume);
        stream.set_total_volume(-0.2);
        assert_eq!(0.0, stream.total_volume);
        stream.set_total_volume(1.2);
        assert_eq!(1.0, stream.total_volume);

        stream.set_partial_volume(0.3, 0, 0);
        assert_eq!(1.0, stream.total_volume);

        assert!(stream.is_empty());
        assert!(stream.get_current_playing().is_empty());

        // Empty
        // merge(&mut self, other: Stream)
        // sync(&mut self, new: Stream)
        // merge_parallel(&mut self, other: Stream)

        stream.play();
        stream.pause();
        stream.stop();

        assert_eq!(0.0, stream.get_position());
        stream.update();
        stream.goto_track(0, 0);

        assert!(stream.get_threads().is_empty());
    }

    #[test]
    #[ignore = "need a sample audio in the repository"]
    fn stream() {
        let mut stream = Stream::from_source(
            Box::new(LocalOpener::new("test/Quick Metal Riff 1.mp3".to_string())),
            1.0,
        )
        .unwrap();

        assert_eq!(1.0, stream.total_volume);
        stream.set_total_volume(0.6);
        assert_eq!(0.6, stream.total_volume);
        stream.set_total_volume(-0.2);
        assert_eq!(0.0, stream.total_volume);
        stream.set_total_volume(1.2);
        assert_eq!(1.0, stream.total_volume);

        stream.set_partial_volume(0.3, 0, 0);
        assert_eq!(1.0, stream.total_volume);

        assert!(stream.is_empty());
        assert!(stream.get_current_playing().is_empty());

        // Empty
        // merge(&mut self, other: Stream)
        // sync(&mut self, new: Stream)
        // merge_parallel(&mut self, other: Stream)

        stream.play();
        stream.pause();
        stream.stop();

        assert_eq!(0.0, stream.get_position());
        stream.update();
        stream.goto_track(0, 0);

        assert!(stream.get_threads().is_empty());
    }
}
