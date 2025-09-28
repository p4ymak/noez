use rodio::{SampleRate, Sink, source::Pink};
use std::{thread, time::Duration};

mod stereo;
use stereo::Stereo;
const RATE: SampleRate = 48000;
const STEREO_SPREAD: f32 = 0.7;

fn main() {
    let audio =
        rodio::OutputStreamBuilder::open_default_stream().expect("open default audio stream");
    let sink = Sink::connect_new(audio.mixer());
    let stereo = Stereo::new(Pink::new(RATE), Pink::new(RATE), STEREO_SPREAD);
    sink.append(stereo);

    println!("\x1b[105mEnjoy Pink Noise in Stereo\x1b[0m");
    thread::sleep(Duration::MAX);
}
