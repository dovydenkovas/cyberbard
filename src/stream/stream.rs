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

use std::sync::Mutex;
use std::time::Duration;

use crate::stream::Opener;
use crate::stream::trackstream::TrackStream;

use super::threadstream::ThreadStream;

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

    pub fn from_source(src: Box<dyn Opener + Send>, volume: f32) -> Stream {
        let pl = ThreadStream::new(
            &mut OSTREAM.lock().unwrap(),
            vec![TrackStream::new(src, volume)],
            1.0,
        )
        .unwrap();
        Stream {
            threads: vec![pl],
            total_volume: 0.0,
        }
    }

    pub fn set_total_volume(&mut self, volume: f32) {
        self.total_volume = volume;
        for pl in self.threads.iter_mut() {
            pl.update_volume(volume);
        }
    }

    pub fn set_partial_volume(&mut self, volume: f32, thread_index: usize, audio_index: usize) {
        self.threads[thread_index].set_partial_volume(volume, audio_index);
    }

    pub fn is_empty(&self) -> bool {
        self.threads.len() == 0
    }

    pub fn get_threads(self) -> Vec<ThreadStream> {
        self.threads
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

    pub fn get_position(&self) -> f32 {
        match self.threads.get(0) {
            Some(pl) => pl.get_position(),
            None => 0.0,
        }
    }

    pub fn update(&mut self) {
        for thread in self.threads.iter_mut() {
            thread.update();
        }
        std::thread::sleep(Duration::from_millis(5));
    }
}
