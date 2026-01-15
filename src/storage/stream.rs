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
    fn source(&mut self) -> Result<Box<dyn rodio::Source + Send>, Box<dyn std::error::Error>>;
    fn total_duration(&self) -> f32;
}

pub struct SubStream {
    source: Box<dyn Opener + Send>,
    volume: f32,
}

impl SubStream {
    pub fn new(source: Box<dyn Opener + Send>, volume: f32) -> SubStream {
        SubStream { source, volume }
    }

    pub fn reset_sink(&mut self, sink: &mut Sink) -> Result<(), Box<dyn std::error::Error>> {
        let source = self.source.source()?;
        sink.clear();
        sink.append(source);
        Ok(())
    }

    pub fn get_volume(&self) -> f32 {
        self.volume
    }

    pub fn set_volume(&mut self, v: f32) {
        self.volume = v;
    }

    pub fn total_duration(&self) -> f32 {
        self.source.total_duration()
    }
}

pub struct Playlist {
    sources: Vec<SubStream>,
    current: usize,
    sink: Sink,
    is_stopped: bool,
    volume: f32,
}

impl Playlist {
    pub fn new(
        ostream: &mut OutputStream,
        mut sources: Vec<SubStream>,
        volume: f32,
    ) -> Option<Playlist> {
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
                volume,
            })
        }
    }

    pub fn replace_sources(&mut self, sources: Vec<SubStream>) {
        if sources.len() == 0 {
            unreachable!("Empty sources replacement");
        }

        while self.current > sources.len() {
            self.current -= 1;
        }

        self.sources = sources;
        let pos = self.sink.get_pos();
        self.sources[self.current].reset_sink(&mut self.sink);
        self.sink.try_seek(pos).unwrap();
        if !self.is_stopped {
            self.play();
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
            self.update_volume(self.volume);
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

    pub fn get_position(&self) -> f32 {
        self.sink.get_pos().as_secs_f32() / self.sources[self.current].total_duration()
    }

    pub fn update_volume(&mut self, volume: f32) {
        self.volume = volume;
        self.sink
            .set_volume(volume * self.sources[self.current].get_volume());
    }

    pub fn set_partial_volume(&mut self, vol: f32, index: usize) {
        self.sources[index].set_volume(vol);
        if index == self.current {
            self.update_volume(self.volume);
        }
    }

    pub fn extend(&mut self, other: Playlist) {
        self.sources.extend(other.sources);
    }
}

pub struct Stream {
    playlists: Vec<Playlist>,
    total_volume: f32,
}

impl Stream {
    pub fn new(playlists: Vec<Playlist>, total_volume: f32) -> Stream {
        Stream {
            playlists,
            total_volume,
        }
    }

    pub fn from_source(src: Box<dyn Opener + Send>, volume: f32) -> Stream {
        let pl = Playlist::new(
            &mut OSTREAM.lock().unwrap(),
            vec![SubStream::new(src, volume)],
            1.0,
        )
        .unwrap();
        Stream {
            playlists: vec![pl],
            total_volume: 0.0,
        }
    }

    pub fn set_total_volume(&mut self, volume: f32) {
        self.total_volume = volume;
        for pl in self.playlists.iter_mut() {
            pl.update_volume(volume);
        }
    }

    pub fn set_partial_volume(&mut self, volume: f32, playlist: usize, audio_index: usize) {
        self.playlists[playlist].set_partial_volume(volume, audio_index);
    }

    pub fn is_empty(&self) -> bool {
        self.playlists.len() == 0
    }

    pub fn get_playlists(self) -> Vec<Playlist> {
        self.playlists
    }

    pub fn merge(&mut self, other: Stream) {
        for (i, pl) in other.playlists.into_iter().enumerate() {
            if i < self.playlists.len() {
                self.playlists[i].extend(pl);
            } else {
                self.playlists.extend(vec![pl]);
            }
        }
    }

    pub fn sync(&mut self, new: Stream) {
        self.total_volume = new.total_volume;
        for (i, pl) in new.playlists.into_iter().enumerate() {
            if i < self.playlists.len() {
                self.playlists[i].replace_sources(pl.sources);
                self.playlists[i].update_volume(self.total_volume);
            } else {
                self.playlists.extend(vec![pl]);
                self.playlists[i].update_volume(self.total_volume);
            }
        }
    }

    pub fn merge_parallel(&mut self, other: Stream) {
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

    pub fn get_position(&self) -> f32 {
        match self.playlists.get(0) {
            Some(pl) => pl.get_position(),
            None => 0.0,
        }
    }

    pub fn update(&mut self) {
        for playlist in self.playlists.iter_mut() {
            playlist.update();
        }
        std::thread::sleep(Duration::from_millis(5));
    }
}
