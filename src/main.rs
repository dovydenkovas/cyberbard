mod audio;
mod stream;
mod source;
mod composition;
mod track;
mod storage;
mod tag;
// mod player;

/// Application entry point.
/// Initialize all structures and start player and application threads.

use std::time::Duration;
use std::thread;

use rodio::Source;

fn mixer() -> Result<(), Box<dyn std::error::Error>> {
    let stream_handle = rodio::OutputStreamBuilder::open_default_stream()?;
    let sink = rodio::Sink::connect_new(stream_handle.mixer());
    let (controller, mixer) = rodio::mixer::mixer(2, 44_100);

    println!("!!!");
    let file = std::fs::File::open("music/2.mp3")?;
    let src = rodio::Decoder::try_from(file)?;

    controller.add(src);
    println!("!!!");
    let file = std::fs::File::open("music/1.mp3")?;
    controller.add(rodio::Decoder::try_from(file)?);
    let file = std::fs::File::open("music/3.mp3")?;
    let src = rodio::Decoder::try_from(file)?;
    controller.add(src.delay(Duration::from_secs(1)).repeat_infinite());


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


fn sinks() -> Result<(), Box<dyn std::error::Error>> {
    let stream_handle = rodio::OutputStreamBuilder::open_default_stream()?;
    let s1 = rodio::Sink::connect_new(stream_handle.mixer());
    let s2 = rodio::Sink::connect_new(stream_handle.mixer());

    let file = std::fs::File::open("music/2.mp3")?;
    s1.append(rodio::Decoder::try_from(file)?);

    let file = std::fs::File::open("music/1.mp3")?;
    s2.append(rodio::Decoder::try_from(file)?);

    println!("!!!");
    std::thread::sleep(std::time::Duration::from_secs(2));
    // sink.try_seek(Duration::from_secs(20))?;
    s1.play();
    s2.play();


    // std::thread::sleep(std::time::Duration::from_secs(2));
    // sink.try_seek(Duration::from_secs(20))?;

    while !s1.empty() {
        println!("{:?} {:?}", s1.get_pos(), s2.get_pos());
    }

    // This doesn't do anything since the sound has ended already.
    // s1.try_seek(Duration::from_secs(5))?;
    // println!("seek example ended");

    Ok(())
}

fn main() {
    let _ = mixer();
}
