use desktop::audio::{self};
use ironboy_core::{gb::GameBoy, JoypadButton, FPS, VIEWPORT_HEIGHT, VIEWPORT_WIDTH};
use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::Canvas, video::Window};
use std::{
    env,
    fs::File,
    io::Read,
    time::{self, Instant},
};

const SCALE: u32 = 4;
const WINDOW_WIDTH: u32 = (VIEWPORT_WIDTH as u32) * SCALE;
const WINDOW_HEIGHT: u32 = (VIEWPORT_HEIGHT as u32) * SCALE;
const FRAME_DURATION_NS: f32 = 1_000_000_000.0 / FPS;
const FRAME_DURATION: std::time::Duration = std::time::Duration::from_nanos(FRAME_DURATION_NS as u64);

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Invalid input");
        return;
    }

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Iron Boy", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .resizable()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().accelerated().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let (audio_interface, mut _audio_device) = audio::create_audio_device(&sdl_context).unwrap();

    let mut rom = File::open(&args[1]).expect("Unable to open file");
    let mut buffer = Vec::new();
    rom.read_to_end(&mut buffer).expect("Issue while reading file");
    let mut game_boy = GameBoy::new(&args[1], buffer, audio_interface, false);

    let mut overshoot = 0;
    'game: loop {
        let frame_start_time = std::time::Instant::now();

        overshoot = game_boy.run(overshoot);
        let data = game_boy.ppu_buffer().to_vec();
        recalculate_screen(&mut canvas, &data);

        let time_elapsed = frame_start_time.elapsed();
        let delay = FRAME_DURATION.checked_sub(time_elapsed);
        match delay {
            None => {}
            Some(delay) => spin_sleep::sleep(delay),
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'game,
                Event::KeyDown { keycode, .. } => match keycode {
                    Some(Keycode::X) => game_boy.button_down(JoypadButton::A),
                    Some(Keycode::Z) => game_boy.button_down(JoypadButton::B),
                    Some(Keycode::Return) => game_boy.button_down(JoypadButton::Select),
                    Some(Keycode::Space) => game_boy.button_down(JoypadButton::Start),
                    Some(Keycode::Up) => game_boy.button_down(JoypadButton::Up),
                    Some(Keycode::Left) => game_boy.button_down(JoypadButton::Left),
                    Some(Keycode::Down) => game_boy.button_down(JoypadButton::Down),
                    Some(Keycode::Right) => game_boy.button_down(JoypadButton::Right),
                    _ => {}
                },
                Event::KeyUp { keycode, .. } => match keycode {
                    Some(Keycode::X) => game_boy.button_up(JoypadButton::A),
                    Some(Keycode::Z) => game_boy.button_up(JoypadButton::B),
                    Some(Keycode::Return) => game_boy.button_up(JoypadButton::Select),
                    Some(Keycode::Space) => game_boy.button_up(JoypadButton::Start),
                    Some(Keycode::Up) => game_boy.button_up(JoypadButton::Up),
                    Some(Keycode::Left) => game_boy.button_up(JoypadButton::Left),
                    Some(Keycode::Down) => game_boy.button_up(JoypadButton::Down),
                    Some(Keycode::Right) => game_boy.button_up(JoypadButton::Right),
                    _ => {}
                },
                _ => {}
            };
        }
    }
}

fn recalculate_screen(canvas: &mut Canvas<Window>, data: &[(u8, u8, u8)]) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    for x in 0..VIEWPORT_WIDTH {
        for y in 0..VIEWPORT_HEIGHT {
            let i = y * VIEWPORT_WIDTH + x;
            let color = data[i as usize];
            canvas.set_draw_color(Color::RGB(color.0, color.1, color.2));
            let rect = Rect::new(
                (x as u32 * SCALE) as i32,
                (y as u32 * SCALE) as i32,
                SCALE + 4, // change this if you want line speration
                SCALE + 4, // change this if you want line speration
            );
            canvas.fill_rect(rect).unwrap();
        }
    }

    canvas.present();
}
