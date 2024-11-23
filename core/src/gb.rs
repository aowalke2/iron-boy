use std::{cell::RefCell, rc::Rc};

use crate::{
    audio::AudioInterface,
    bus::Bus,
    cartridge::Cartridge,
    cpu::{registers::Registers, Cpu, CPU_CLOCK_SPEED},
    scheduler::{event::EventType, Scheduler},
    GameBoyMode, JoypadButton, FPS,
};

const CYCLES_PER_FRAME: usize = (CPU_CLOCK_SPEED as f32 / FPS) as usize;

pub struct GameBoy {
    pub cpu: Cpu,
    scheduler: Rc<RefCell<Scheduler>>,
    audio_interface: AudioInterface,
    game_title: String,
    frame: Option<Box<[u8]>>,
}

impl GameBoy {
    pub fn new(rom_name: &str, buffer: Vec<u8>, audio_interface: AudioInterface, skip_boot: bool) -> GameBoy {
        let cartridge = Cartridge::load(rom_name.into(), buffer).unwrap();
        let game_title = cartridge.title().to_string();
        let scheduler = Rc::new(RefCell::new(Scheduler::new()));
        GameBoy {
            cpu: Cpu::new(
                Bus::new(cartridge, scheduler.clone(), audio_interface.sampling_frequency() as f32),
                Registers::new(GameBoyMode::Monochrome, skip_boot),
            ),
            scheduler,
            audio_interface,
            game_title,
            frame: Some(vec![0; 160 * 144 * 4].into_boxed_slice()),
        }
    }

    pub fn cycle(&mut self) -> u32 {
        self.cpu.cycle()
    }

    pub fn run(&mut self, overshoot: usize) -> usize {
        let start_time = self.scheduler.borrow().timestamp();
        let end_time = start_time + CYCLES_PER_FRAME - overshoot;

        self.scheduler.borrow_mut().schedule_at(EventType::FrameComplete, end_time);
        'game: loop {
            while self.scheduler.borrow().timestamp() <= self.scheduler.borrow().timestamp_of_next_event() {
                let cycles = self.cpu.cycle() as usize;
                self.scheduler.borrow_mut().update(cycles);
            }

            if self.handle_events() {
                break 'game;
            }
        }

        self.scheduler.borrow().timestamp() - start_time
    }

    fn handle_events(&mut self) -> bool {
        let mut scheduler = self.scheduler.borrow_mut();
        while let Some((event, timestamp)) = scheduler.pop() {
            let future_event = match event {
                EventType::FrameComplete => return true,
                EventType::Timer(timer_event) => self.cpu.bus.timer.handle_event(timer_event, timestamp),
                EventType::Ppu(ppu_event) => self.cpu.bus.ppu.handle_event(ppu_event),
                EventType::Apu(apu_event) => self.cpu.bus.apu.handle_event(apu_event, &mut self.audio_interface),
            };
            if let Some((event_type, time)) = future_event {
                scheduler.schedule_at(event_type, timestamp + time);
            }
        }
        false
    }

    pub fn ppu_buffer(&self) -> &[(u8, u8, u8)] {
        &self.cpu.bus.ppu.screen_buffer
    }

    pub fn button_up(&mut self, button: JoypadButton) {
        self.cpu.bus.joy_pad.button_up(button)
    }

    pub fn button_down(&mut self, button: JoypadButton) {
        self.cpu.bus.joy_pad.button_down(button)
    }
}
