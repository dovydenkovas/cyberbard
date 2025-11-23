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

use crate::storage::stream::{Playlist, Stream, SubStream};

enum Command {
    Play,
    Pause,
    Stop,
    GetPosition,
    SetStream(Stream),
}

enum Response {
    Position(Duration),
    // Ok,
    // Err
}

/// Music Player.
/// Get audio Stream and control play process.
pub struct Player {
    cmd_tx: Sender<Command>,
    resp_rx: Receiver<Response>,
    handle: Option<thread::JoinHandle<()>>,
    paused: bool,
}

impl Player {
    pub fn new(mut stream: Stream) -> Player {
        let (cmd_tx, cmd_rx): (Sender<Command>, Receiver<Command>) = mpsc::channel();
        let (resp_tx, resp_rx): (Sender<Response>, Receiver<Response>) = mpsc::channel();

        let handle = thread::spawn(move || {
            loop {
                stream.update();
                match cmd_rx.try_recv() {
                    Ok(Command::Play) => stream.play(),

                    Ok(Command::Pause) => stream.pause(),

                    Ok(Command::Stop) => {
                        stream.stop();
                        stream.pause();
                    }

                    Ok(Command::SetStream(s)) => stream = s,

                    Ok(Command::GetPosition) => {
                        resp_tx
                            .send(Response::Position(Duration::from_millis(50)))
                            .unwrap();
                    }
                    Err(mpsc::TryRecvError::Empty) => {
                        thread::sleep(Duration::from_millis(50));
                    }
                    Err(mpsc::TryRecvError::Disconnected) => {
                        break;
                    }
                }
            }
        });

        Player {
            cmd_tx,
            resp_rx,
            handle: Some(handle),
            paused: true,
        }
    }

    pub fn set_stream(&mut self, stream: Stream) {
        let _ = self.cmd_tx.send(Command::SetStream(stream));
    }

    pub fn play(&mut self) {
        let _ = self.cmd_tx.send(Command::Play);
    }

    pub fn pause(&mut self) {
        let _ = self.cmd_tx.send(Command::Pause);
    }

    pub fn is_paused(&self) -> bool {
        self.paused
    }

    pub fn stop(&mut self) {
        let _ = self.cmd_tx.send(Command::Stop);
    }

    pub fn get_position(&self) -> Duration {
        let _ = self.cmd_tx.send(Command::GetPosition);
        match self.resp_rx.recv() {
            Err(_) => Duration::from_millis(0),
            Ok(Response::Position(pos)) => pos,
        }
    }

    pub fn get_volume(&self) -> u8 {
        0
    }

    pub fn set_volume(&mut self, vol: u8) {}
}
