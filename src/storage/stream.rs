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

use rodio::OutputStream;
use rodio::Sink;
use std::sync::Mutex;
use std::time::Duration;

lazy_static::lazy_static! {
static ref OSTREAM: Mutex<rodio::OutputStream> =
    Mutex::new(rodio::OutputStreamBuilder::open_default_stream().unwrap());
}

pub trait Opener {
    fn source(&self) -> Result<Box<dyn rodio::Source + Send>, Box<dyn std::error::Error>>;
}

pub struct SubStream {
    source: Box<dyn Opener + Send>,
}

impl SubStream {
    pub fn new(source: Box<dyn Opener + Send>) -> SubStream {
        SubStream { source }
    }

    pub fn reset_sink(&self, sink: &mut Sink) -> Result<(), Box<dyn std::error::Error>> {
        let source = self.source.source()?;
        sink.clear();
        sink.append(source);
        Ok(())
    }
}

pub struct Playlist {
    sources: Vec<SubStream>,
    current: usize,
    sink: Sink,
    is_stopped: bool,
}

impl Playlist {
    pub fn new(ostream: &mut OutputStream, sources: Vec<SubStream>) -> Option<Playlist> {
        if sources.len() == 0 {
            None
        } else {
            let mut sink = Sink::connect_new(ostream.mixer());
            sources[0].reset_sink(&mut sink);
            Some(Playlist {
                sources,
                current: 0,
                sink,
                is_stopped: true,
            })
        }
    }

    pub fn play(&mut self) {
        self.sink.play();
        self.is_stopped = false;
    }

    fn next_sink_if_need(&mut self) {
        if !self.is_stopped && self.sink.empty() {
            self.sink.stop();
            self.current = (self.current + 1) % self.sources.len();
            self.sources[self.current].reset_sink(&mut self.sink);
            self.play();
        }
    }

    pub fn update(&mut self) {
        self.next_sink_if_need();
    }

    pub fn pause(&mut self) {
        self.next_sink_if_need();
        self.sink.pause();
    }

    pub fn stop(&mut self) {
        self.sink.stop();
        self.sink.clear();
        self.current = 0;
        self.is_stopped = true;
    }
}

pub struct Stream {
    playlists: Vec<Playlist>,
}

impl Stream {
    pub fn new(playlists: Vec<Playlist>) -> Stream {
        Stream { playlists }
    }

    pub fn from_source(src: Box<dyn Opener + Send>) -> Stream {
        let pl = Playlist::new(&mut OSTREAM.lock().unwrap(), vec![SubStream::new(src)]).unwrap();
        Stream {
            playlists: vec![pl],
        }
    }

    pub fn is_empty(&self) -> bool {
        self.playlists.len() == 0
    }

    pub fn get_playlists(self) -> Vec<Playlist> {
        self.playlists
    }

    pub fn merge(&mut self, other: Stream) {
        self.playlists.extend(other.playlists);
    }

    pub fn play(&mut self) {
        for playlist in self.playlists.iter_mut() {
            playlist.play();
        }
    }

    pub fn pause(&mut self) {
        for playlist in self.playlists.iter_mut() {
            playlist.pause();
        }
    }

    pub fn stop(&mut self) {
        for playlist in self.playlists.iter_mut() {
            playlist.stop();
        }
    }

    pub fn update(&mut self) {
        for playlist in self.playlists.iter_mut() {
            playlist.update();
        }
        std::thread::sleep(Duration::from_millis(5));
    }
}
