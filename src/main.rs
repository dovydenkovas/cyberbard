mod audio;
mod composition;
mod gui;
mod source;
mod stream;
mod track;

fn main() {
    gui::window::run_gui();
    println!("Hello, world!");
}
