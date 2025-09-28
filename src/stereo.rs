use rodio::Source;
use std::time::Duration;

pub struct Stereo<L: Source, R: Source> {
    wide: f32,
    pan: bool,
    left: L,
    right: R,
}

impl<L: Source, R: Source> Stereo<L, R> {
    pub fn new(left: L, right: R, wide: f32) -> Self {
        Stereo {
            wide: (1.0 - wide.clamp(0.0, 1.0)) * 0.5,
            pan: false,
            left,
            right,
        }
    }
}

impl<L: Source, R: Source> Iterator for Stereo<L, R> {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        self.pan = !self.pan;
        let wide = self.wide;
        if self.pan {
            self.left
                .next()
                .and_then(|l| self.right.next().map(|r| r * wide + l * (1.0 - wide)))
        } else {
            self.right
                .next()
                .and_then(|r| self.left.next().map(|l| l * wide + r * (1.0 - wide)))
        }
    }
}

impl<L: Source, R: Source> Source for Stereo<L, R> {
    fn current_span_len(&self) -> Option<usize> {
        self.left.current_span_len()
    }

    fn channels(&self) -> rodio::ChannelCount {
        2
    }

    fn sample_rate(&self) -> rodio::SampleRate {
        self.left.sample_rate()
    }

    fn total_duration(&self) -> Option<Duration> {
        self.left.total_duration()
    }
}
