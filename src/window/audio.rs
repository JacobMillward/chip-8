use std::time::Duration;

use rodio::{source::SineWave, OutputStream, OutputStreamHandle, Source};

pub struct AudioBuzzer {
    stream_handle: OutputStreamHandle,
    _stream: OutputStream,
    frequency: f32,
    amplitude: f32,
}

impl AudioBuzzer {
    pub fn new() -> Self {
        let (stream, stream_handle) = OutputStream::try_default().unwrap();

        Self {
            stream_handle,
            _stream: stream,
            frequency: 440.0,
            amplitude: 0.25,
        }
    }

    pub fn play(&self) -> Result<(), rodio::PlayError> {
        let source = SineWave::new(self.frequency)
            .amplify(self.amplitude)
            .take_duration(Duration::from_secs_f32(0.1));

        self.stream_handle.play_raw(source)?;

        Ok(())
    }
}
