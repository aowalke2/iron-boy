use ironboy_core::audio::{audio_buffer::SampleConsumer, AudioInterface};
use ringbuf::traits::Consumer;
use sdl2::audio::{AudioCallback, AudioDevice, AudioFormat, AudioSpec, AudioSpecDesired};

pub struct GbAudioCallback {
    consumer: SampleConsumer,
    spec: AudioSpec,
}

impl AudioCallback for GbAudioCallback {
    type Channel = i16;

    fn callback(&mut self, output_samples: &mut [i16]) {
        let samples = self.consumer.pop_slice(output_samples);
        for sample in output_samples.iter_mut().skip(samples) {
            *sample = self.spec.silence as i16
        }
    }
}

pub fn create_audio_device(sdl: &sdl2::Sdl) -> Result<(AudioInterface, AudioDevice<GbAudioCallback>), String> {
    let audio_spec_desired = AudioSpecDesired {
        freq: Some(44100),
        channels: Some(2),
        samples: None,
    };

    let audio_subsystem = sdl.audio()?;
    let mut freq = 0;
    let mut audio = None;
    let device = audio_subsystem.open_playback(None, &audio_spec_desired, |spec| {
        freq = spec.freq;
        if spec.format != AudioFormat::S16LSB {
            panic!("Unsupported audio format {:?}", spec.format);
        }

        let samples_per_channel = (spec.samples as usize) * 2;
        let buffer_capacity = (spec.channels as usize) * samples_per_channel;
        let (audio_device, consumer) = AudioInterface::create_channel(freq, Some(buffer_capacity));
        audio = Some(audio_device);
        GbAudioCallback { consumer, spec }
    })?;

    device.resume();
    Ok((audio.take().unwrap(), device))
}
