use std::time::Duration;
use std::thread;
use std::sync::mpsc::{self, Sender, Receiver};

use crate::stream::Stream;

enum Command {
    Play,
    Pause,
    Reset,
    Stop,
    SetPosition(Duration),
    GetPosition
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
    pub fn new() -> Player {
        let (cmd_tx, cmd_rx): (Sender<Command>, Receiver<Command>) = mpsc::channel();
        let (resp_tx, resp_rx): (Sender<Response>, Receiver<Response>) = mpsc::channel();

        let handle = thread::spawn(move || {
            loop {
                match cmd_rx.try_recv() {
                    Ok(Command::Play) => {

                    }

                    Ok(Command::GetPosition) => {

                        resp_tx.send(Response::Position(Duration::from_millis(50)));
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
            paused: true
        }

    }

    pub fn set_stream(&self, stream: Stream) {

    }

    pub fn get_stream(&self) -> Stream {

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

    pub fn reset(&mut self) {
        let _ = self.cmd_tx.send(Command::Reset);
    }

    pub fn set_position(&mut self, pos: Duration) {
        let _ = self.cmd_tx.send(Command::SetPosition(pos));
    }

    pub fn get_position(&self) -> Duration {
        let _ = self.cmd_tx.send(Command::GetPosition);
        match self.resp_rx.recv() {
            Err(_) => Duration::from_millis(0),
            Ok(Response::Position(pos)) => pos,
        }
    }

    pub fn get_volume(&self) -> u8 {

    }

    pub fn set_volume(&mut self, vol: u8) {

    }
}



fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stream_handle = rodio::OutputStreamBuilder::open_default_stream()?;
    let sink = rodio::Sink::connect_new(stream_handle.mixer());
    let (controller, mixer) = rodio::mixer::mixer(2, 44_100);

    println!("!!!");
    let file = std::fs::File::open("music/necroTown.mp3")?;
    controller.add(rodio::Decoder::try_from(file)?);
    println!("!!!");
    let file = std::fs::File::open("music/1.mp3")?;
    controller.add(rodio::Decoder::try_from(file)?);

    println!("!!!");
    sink.append(mixer);

    println!("!!!");
    std::thread::sleep(std::time::Duration::from_secs(2));
    // sink.try_seek(Duration::from_secs(20))?;
    sink.play();

    // std::thread::sleep(std::time::Duration::from_secs(2));
    // sink.try_seek(Duration::from_secs(20))?;

    while !sink.empty() {
        println!("{:?}", sink.get_pos());
    }

    // This doesn't do anything since the sound has ended already.
    sink.try_seek(Duration::from_secs(5))?;
    println!("seek example ended");

    Ok(())
}
