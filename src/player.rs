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

use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::Duration;

use crate::stream::stream::Stream;

enum Command {
    Play,
    Pause,
    Stop,
    Reset,
    GetPosition,
    SetStream(Stream),
    SyncStream(Stream),
    SetVolume(f32),
    SetTrackVolume(f32, usize, usize),
}

enum Response {
    Position(f32),
}

/// Music Player.
/// Get audio Stream and control play process.
pub struct Player {
    cmd_tx: Sender<Command>,
    resp_rx: Receiver<Response>,
    paused: bool,
}

impl Player {
    pub fn new() -> Player {
        let (cmd_tx, cmd_rx): (Sender<Command>, Receiver<Command>) = mpsc::channel();
        let (resp_tx, resp_rx): (Sender<Response>, Receiver<Response>) = mpsc::channel();

        let _ = thread::spawn(move || {
            let mut opt_stream: Option<Stream> = None;
            loop {
                match &mut opt_stream {
                    None => match cmd_rx.try_recv() {
                        Ok(Command::SetStream(s)) => opt_stream = Some(s),

                        Ok(Command::GetPosition) => {
                            resp_tx.send(Response::Position(0.0)).unwrap();
                        }
                        Ok(_) => (),
                        Err(mpsc::TryRecvError::Empty) => {
                            thread::sleep(Duration::from_millis(50));
                        }
                        Err(mpsc::TryRecvError::Disconnected) => {
                            break;
                        }
                    },
                    Some(stream) => {
                        stream.update();
                        match cmd_rx.try_recv() {
                            Ok(Command::Play) => stream.play(),

                            Ok(Command::Pause) => stream.pause(),

                            Ok(Command::Stop) => {
                                stream.stop();
                                stream.pause();
                            }

                            Ok(Command::Reset) => {
                                stream.stop();
                                stream.pause();
                                opt_stream = None;
                            }

                            Ok(Command::SetStream(s)) => opt_stream = Some(s),
                            Ok(Command::SyncStream(s)) => stream.sync(s),
                            Ok(Command::GetPosition) => {
                                resp_tx
                                    .send(Response::Position(stream.get_position()))
                                    .unwrap();
                            }
                            Ok(Command::SetVolume(vol)) => {
                                stream.set_total_volume(vol);
                            }
                            Ok(Command::SetTrackVolume(v, p, i)) => {
                                stream.set_partial_volume(v, p, i)
                            }

                            Err(mpsc::TryRecvError::Empty) => {
                                thread::sleep(Duration::from_millis(50));
                            }
                            Err(mpsc::TryRecvError::Disconnected) => {
                                stream.stop();
                                break;
                            }
                        }
                    }
                }
            }
        });

        Player {
            cmd_tx,
            resp_rx,
            paused: true,
        }
    }

    pub fn set_stream(&mut self, stream: Stream) {
        let _ = self.cmd_tx.send(Command::SetStream(stream));
    }

    pub fn sync(&mut self, stream: Stream) {
        let _ = self.cmd_tx.send(Command::SyncStream(stream));
    }

    pub fn play(&mut self) {
        let _ = self.cmd_tx.send(Command::Play);
        self.paused = false;
    }

    pub fn pause(&mut self) {
        let _ = self.cmd_tx.send(Command::Pause);
        self.paused = true;
    }

    pub fn is_paused(&self) -> bool {
        self.paused
    }

    pub fn stop(&mut self) {
        let _ = self.cmd_tx.send(Command::Stop);
        self.paused = true;
    }

    pub fn reset(&mut self) {
        let _ = self.cmd_tx.send(Command::Reset);
        self.paused = true;
    }

    pub fn get_position(&self) -> f32 {
        let _ = self.cmd_tx.send(Command::GetPosition);
        match self.resp_rx.recv() {
            Err(_) => 0.0,
            Ok(Response::Position(pos)) => pos,
        }
    }

    pub fn set_volume(&mut self, vol: f32) {
        let _ = self.cmd_tx.send(Command::SetVolume(vol));
    }

    pub fn set_track_volume(&mut self, volume: f32, playlist_index: usize, index: usize) {
        let _ = self
            .cmd_tx
            .send(Command::SetTrackVolume(volume, playlist_index, index));
    }
}

impl Default for Player {
    fn default() -> Self {
        Self::new()
    }
}
