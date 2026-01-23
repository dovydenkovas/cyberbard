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

use super::trackstream::TrackStream;

pub struct ThreadStream {
    pub tracks: Vec<TrackStream>,
    pub current: usize,
    pub sink: Sink,
    pub is_stopped: bool,
    pub volume: f32,
}

impl ThreadStream {
    pub fn new(
        ostream: &mut OutputStream,
        tracks: Vec<TrackStream>,
        volume: f32,
    ) -> Option<ThreadStream> {
        if tracks.is_empty() {
            None
        } else {
            let mut ts = ThreadStream {
                tracks,
                current: 0,
                sink: Sink::connect_new(ostream.mixer()),
                is_stopped: true,
                volume,
            };
            match ts.goto_next_avaliable() {
                Ok(_) => Some(ts),
                Err(e) => {
                    eprintln!("No available tracks in thread stream. {}", e);
                    None
                }
            }
        }
    }

    pub fn replace_sources(&mut self, sources: Vec<TrackStream>) {
        if sources.is_empty() {
            unreachable!("Empty sources replacement");
        }

        while self.current > sources.len() {
            self.current -= 1;
        }

        self.tracks = sources;
        let pos = self.sink.get_pos();
        if self.tracks[self.current]
            .reset_sink(&mut self.sink)
            .is_err()
        {
            // TODO: Show error message.
            self.goto_next_avaliable().unwrap();
        }
        let _ = self.sink.try_seek(pos); // Just go from begin

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
            self.current = (self.current + 1) % self.tracks.len();
            // TODO: show error message
            self.goto_next_avaliable().unwrap();
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
        self.sink.get_pos().as_secs_f32() / self.tracks[self.current].total_duration()
    }

    pub fn update_volume(&mut self, volume: f32) {
        self.volume = volume;
        self.sink
            .set_volume(volume * self.tracks[self.current].get_volume());
    }

    pub fn set_partial_volume(&mut self, vol: f32, index: usize) {
        self.tracks[index].set_volume(vol);
        if index == self.current {
            self.update_volume(self.volume);
        }
    }

    pub fn extend(&mut self, other: ThreadStream) {
        self.tracks.extend(other.tracks);
    }

    fn goto_next_avaliable(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut tries_counter = 0;

        while self.current < self.tracks.len()
            && self.tracks[self.current]
                .reset_sink(&mut self.sink)
                .is_err()
        {
            self.current = (self.current + 1) % self.tracks.len();
            tries_counter += 1;
            if tries_counter == self.tracks.len() {
                return Err("Something went wrong with running next track".into());
            }
        }
        Ok(())
    }
}
