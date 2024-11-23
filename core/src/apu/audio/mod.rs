use audio_buffer::{AudioRingBuffer, SampleConsumer, SampleProducer};
use ringbuf::traits::Producer;

use super::{Sample, AUDIO_BUFFER_THRESHOLD};

pub mod audio_buffer;
pub mod resampler;

pub struct AudioInterface {
    producer: SampleProducer,
    sampling_frequency: i32,
}

impl AudioInterface {
    pub fn create_channel(sampling_frequency: i32, capacity: Option<usize>) -> (AudioInterface, SampleConsumer) {
        let (producer, consumer) = AudioRingBuffer::new_with_capacity(capacity.unwrap_or(AUDIO_BUFFER_THRESHOLD)).split();
        (
            AudioInterface {
                producer,
                sampling_frequency,
            },
            consumer,
        )
    }

    pub fn sampling_frequency(&self) -> i32 {
        self.sampling_frequency
    }

    pub fn push_sample(&mut self, sample: &Sample<i16>) {
        let _ = self.producer.try_push(sample[0]);
        let _ = self.producer.try_push(sample[0]);
    }
}
