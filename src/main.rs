mod stereo;

use rodio::{SampleRate, Sink, source::noise::Pink};
use std::{thread, time::Duration};
use stereo::Stereo;

const RATE: SampleRate = 48000;
const STEREO_SPREAD: f32 = 0.7;

fn main() {
    std::thread::spawn(|| {
        let audio =
            rodio::OutputStreamBuilder::open_default_stream().expect("open default audio stream");
        let sink = Sink::connect_new(audio.mixer());

        let left = Pink::new(RATE);
        let right = Pink::new(RATE);
        let stereo = Stereo::new(left, right, STEREO_SPREAD);
        sink.append(stereo);

        thread::sleep(Duration::MAX);
    });

    println!("\x1b[105mEnjoy Pink Noise in Stereo..\x1b[0m Press ENTER to Stop.");

    std::io::stdin().read_line(&mut String::new()).ok();
}
