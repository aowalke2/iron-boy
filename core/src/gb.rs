use std::{cell::RefCell, rc::Rc};

use crate::{
    bus::Bus,
    cartridge::Cartridge,
    cpu::{registers::Registers, Cpu},
    scheduler::{self, event::EventType, Scheduler},
    GameBoyMode, JoypadButton,
};

pub struct GameBoy {
    cpu: Cpu,
    scheduler: Rc<RefCell<Scheduler>>,
    game_title: String,
    frame: Option<Box<[u8]>>,
    volume: u8,
}

impl GameBoy {
    pub fn new_dmg(rom_name: &str, buffer: Vec<u8>, skip_boot: bool) -> GameBoy {
        let cartridge = Cartridge::load(rom_name.into(), buffer).unwrap();
        let game_title = cartridge.title().to_string();
        let scheduler = Rc::new(RefCell::new(Scheduler::new()));
        GameBoy {
            cpu: Cpu::new(Bus::new(cartridge, scheduler.clone()), Registers::new(GameBoyMode::Monochrome, skip_boot)),
            scheduler,
            game_title,
            frame: Some(vec![0; 160 * 144 * 4].into_boxed_slice()),
            volume: 50,
        }
    }

    pub fn cycle(&mut self) -> u32 {
        self.cpu.cycle()
    }

    pub fn run(&mut self, cycles: usize) -> usize {
        let start_time = self.scheduler.borrow().timestamp();
        let end_time = start_time + cycles;

        self.scheduler.borrow_mut().schedule_at(EventType::FrameComplete, end_time);
        'game: loop {
            while self.scheduler.borrow().is_empty() || self.scheduler.borrow().timestamp() <= self.scheduler.borrow().timestamp_of_next_event() {
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
        while let Some((event, timestamp)) = self.scheduler.borrow_mut().pop() {
            let future_event = match event {
                EventType::FrameComplete => return true,
                EventType::Timer(timer_event) => self.cpu.bus.timer.on_event(timer_event, timestamp),
                EventType::Gpu(ppu_event) => None,
                EventType::Apu(apu_event) => None,
            };
            if let Some((event_type, time)) = future_event {
                self.scheduler.borrow_mut().schedule_at(event_type, timestamp + time);
            }
        }
        false
    }

    pub fn update_ppu(&mut self) -> bool {
        let result = self.cpu.bus.ppu.screen_updated;
        self.cpu.bus.ppu.screen_updated = false;
        result
    }

    pub fn ppu_buffer(&self) -> &[(u8, u8, u8)] {
        &self.cpu.bus.ppu.screen_buffer
    }

    pub fn increase_volume(&mut self) {
        if self.volume > 95 {
            return;
        }
        self.volume += 5;
    }

    pub fn decrease_volume(&mut self) {
        if self.volume < 5 {
            return;
        }
        self.volume -= 5;
    }

    pub fn button_up(&mut self, button: JoypadButton) {
        self.cpu.bus.joy_pad.button_up(button)
    }

    pub fn button_down(&mut self, button: JoypadButton) {
        self.cpu.bus.joy_pad.button_down(button)
    }
}
