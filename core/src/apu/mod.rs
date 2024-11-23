use audio::{resampler::CosineResampler, AudioInterface};
use channel::Channel;
use frame_sequencer::FrameSequencer;
use mixer::Mixer;
use noise::NoiseChannel;
use square::SquareChannel;
use wave::WaveChannel;

use crate::{
    bus::MemoryAccess,
    cpu::CPU_CLOCK_SPEED,
    scheduler::{
        event::{ApuEvent, EventType, FutureEvent},
        Scheduler,
    },
};
use std::{cell::RefCell, rc::Rc};

pub mod audio;
mod channel;
mod frame_sequencer;
mod mixer;
mod noise;
mod square;
mod wave;

pub const SAMPLING_RATE: u16 = 1024;
pub const AUDIO_BUFFER_THRESHOLD: usize = SAMPLING_RATE as usize * 4;
pub const APU_CLOCK_SPEED: u16 = 512;

pub type Sample<T> = [T; 2];

pub struct Apu {
    ch1: SquareChannel,
    ch2: SquareChannel,
    ch3: WaveChannel,
    ch4: NoiseChannel,
    frame_sequencer: FrameSequencer,
    mixer: Mixer,
    pub right_volume: u8,
    pub left_volume: u8,
    enabled: bool,
    scheduler: Rc<RefCell<Scheduler>>,
    cycles_per_sample: usize,
    resampler: CosineResampler,
    output_buffer: Vec<Sample<f32>>,
}

impl MemoryAccess for Apu {
    fn read_8(&mut self, address: u16) -> u8 {
        match address {
            0xFF10..=0xFF14 => self.ch1.read_8(address),
            0xFF16..=0xFF19 => self.ch2.read_8(address),
            0xFF1A..=0xFF1E => self.ch3.read_8(address),
            0xFF20..=0xFF23 => self.ch4.read_8(address),
            0xFF24 => self.master_volume(),
            0xFF25 => self.mixer.read(),
            0xFF26 => self.master_control(),
            0xFF30..=0xFF3F => self.ch3.read_8(address),
            _ => 0xFF,
        }
    }

    fn write_8(&mut self, address: u16, value: u8) {
        if address == 0xFF26 {
            self.set_master_control(value);
            return;
        }

        if !self.enabled {
            return;
        }

        match address {
            0xFF10..=0xFF14 => self.ch1.write_8(address, value),
            0xFF16..=0xFF19 => self.ch2.write_8(address, value),
            0xFF1A..=0xFF1E => self.ch3.write_8(address, value),
            0xFF20..=0xFF23 => self.ch4.write_8(address, value),
            0xFF24 => self.set_master_volume(value),
            0xFF25 => self.mixer.write(value),
            0xFF26 => {}
            0xFF30..=0xFF3F => self.ch3.write_8(address, value),
            _ => {}
        }
    }
}

impl Apu {
    pub fn new(scheduler: Rc<RefCell<Scheduler>>, sampling_frequency: f32) -> Self {
        let cycles_per_sample = (CPU_CLOCK_SPEED as f32 / sampling_frequency) as usize;
        let resampler = CosineResampler::new(32768.0, sampling_frequency);
        scheduler
            .borrow_mut()
            .schedule((EventType::Apu(ApuEvent::Sample), cycles_per_sample as usize));
        Self {
            ch1: SquareChannel::new(true),
            ch2: SquareChannel::new(false),
            ch3: WaveChannel::new(),
            ch4: NoiseChannel::new(),
            frame_sequencer: FrameSequencer::new(),
            mixer: Mixer::new(),
            right_volume: 0,
            left_volume: 0,
            enabled: false,
            scheduler,
            cycles_per_sample,
            resampler,
            output_buffer: Vec::with_capacity(SAMPLING_RATE as usize),
        }
    }

    pub fn handle_event(&mut self, apu_event: ApuEvent, audio_interface: &mut AudioInterface) -> Option<FutureEvent> {
        let (event, cycles) = match apu_event {
            ApuEvent::LengthTimer => unimplemented!(),
            ApuEvent::Sweep => unimplemented!(),
            ApuEvent::VolumeEnvelope => unimplemented!(),
            ApuEvent::Channel1 => unimplemented!(),
            ApuEvent::Channel2 => unimplemented!(),
            ApuEvent::Channel3 => unimplemented!(),
            ApuEvent::Channel4 => unimplemented!(),
            ApuEvent::Sample => self.handle_sample(audio_interface),
        };
        Some((EventType::Apu(event), cycles))
    }

    fn handle_sample(&mut self, audio_interface: &mut AudioInterface) -> (ApuEvent, usize) {
        let (output_left, output_right) = self
            .mixer
            .mix([self.ch1.output(), self.ch2.output(), self.ch3.output(), self.ch4.output()]);
        let sample = [output_left as f32, output_right as f32];
        self.resampler.feed(&sample, &mut self.output_buffer);
        self.output_buffer.drain(..).for_each(|[left, right]| {
            audio_interface.push_sample(&[
                (left.round() as i16) * (std::i16::MAX / 512),
                (right.round() as i16) * (std::i16::MAX / 512),
            ]);
        });

        (ApuEvent::Sample, self.cycles_per_sample)
    }

    pub fn cycle(&mut self, ticks: u32) {
        if !self.enabled {
            return;
        }

        self.frame_sequencer
            .cycle(ticks, &mut self.ch1, &mut self.ch2, &mut self.ch3, &mut self.ch4);
        self.ch1.cycle(ticks);
        self.ch2.cycle(ticks);
        self.ch3.cycle(ticks);
        self.ch4.cycle(ticks);
    }

    fn master_control(&self) -> u8 {
        (self.enabled as u8) << 7
            | (self.ch4.enabled() as u8) << 3
            | (self.ch3.enabled() as u8) << 2
            | (self.ch2.enabled() as u8) << 1
            | self.ch1.enabled() as u8
    }

    fn set_master_control(&mut self, value: u8) {
        self.enabled = value & 0x80 == 0x80;
        if !self.enabled {
            self.reset();
        }
    }

    fn master_volume(&self) -> u8 {
        let left_volume = (self.left_volume - 1) << 4;
        let right_volume = self.right_volume - 1;
        left_volume | right_volume
    }

    fn set_master_volume(&mut self, value: u8) {
        self.left_volume = ((value & 0x70) >> 4) + 1;
        self.right_volume = (value & 0x07) + 1;
    }

    fn reset(&mut self) {
        self.ch1.reset();
        self.ch2.reset();
        self.ch3.reset();
        self.ch4.reset();
        self.frame_sequencer.reset();
        self.mixer.reset();
        self.left_volume = 0;
        self.right_volume = 0;
    }
}
