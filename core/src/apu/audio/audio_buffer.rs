use ringbuf::{storage::Heap, traits::Split, wrap::caching::Caching, HeapRb, SharedRb};
use std::sync::Arc;

pub type SampleProducer = Caching<Arc<SharedRb<Heap<i16>>>, true, false>;
pub type SampleConsumer = Caching<Arc<SharedRb<Heap<i16>>>, false, true>;

pub struct AudioRingBuffer {
    producer: SampleProducer,
    consumer: SampleConsumer,
}

impl AudioRingBuffer {
    pub fn new_with_capacity(capacity: usize) -> AudioRingBuffer {
        let ring_buffer = HeapRb::<i16>::new(capacity);
        let (producer, consumer) = ring_buffer.split();
        AudioRingBuffer { producer, consumer }
    }

    pub fn producer(&mut self) -> &mut SampleProducer {
        &mut self.producer
    }

    pub fn consumer(&mut self) -> &mut SampleConsumer {
        &mut self.consumer
    }

    pub fn split(self) -> (SampleProducer, SampleConsumer) {
        (self.producer, self.consumer)
    }
}
