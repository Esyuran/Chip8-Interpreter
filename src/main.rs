mod cpu;

extern crate sdl2;

use std::collections::HashMap;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::audio::{AudioCallback, AudioSpecDesired};
use std::time::Duration;
use sdl2::rect::Rect;



pub fn main() {

    let speed :u32 = 600;
    let file_path = "test.ch8";

    let mut key_map: HashMap<Keycode, usize> = HashMap::new();

    key_map.insert(Keycode::Kp1, 0);
    key_map.insert(Keycode::Kp2, 1);
    key_map.insert(Keycode::Kp3, 2);
    key_map.insert(Keycode::C, 3);
    key_map.insert(Keycode::Kp4, 4);
    key_map.insert(Keycode::Kp5, 5);
    key_map.insert(Keycode::Kp6, 6);
    key_map.insert(Keycode::D, 7);
    key_map.insert(Keycode::Kp7, 8);
    key_map.insert(Keycode::Kp8, 9);
    key_map.insert(Keycode::Kp9, 10);
    key_map.insert(Keycode::E, 11);
    key_map.insert(Keycode::A, 12);
    key_map.insert(Keycode::Kp0, 13);
    key_map.insert(Keycode::B, 14);
    key_map.insert(Keycode::F, 15);



    struct SquareWave {
        phase_inc: f32,
        phase: f32,
        volume: f32
    }

    let desired_spec = AudioSpecDesired {
        freq: Some(44100),
        channels: Some(1),  // mono
        samples: None       // default sample size
    };



    impl AudioCallback for SquareWave {
        type Channel = f32;

        fn callback(&mut self, out: &mut [f32]) {
            // Generate a square wave
            for x in out.iter_mut() {
                *x = if self.phase <= 0.5 {
                    self.volume
                } else {
                    -self.volume
                };
                self.phase = (self.phase + self.phase_inc) % 1.0;
            }
        }
    }


    let (mut memory,mut registers ,mut stack, mut graphics, mut input, mut delay_timer, mut sound_timer, mut pc, mut sp, mut index) = cpu::init(file_path);
    let mut delay_clock = 1;



    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let audio_subsystem = sdl_context.audio().unwrap();

    let device = audio_subsystem.open_playback(None, &desired_spec, |spec| {
        // initialize the audio callback
        SquareWave {
            phase_inc: 440.0 / spec.freq as f32,
            phase: 0.0,
            volume: 0.01
        }
    }).unwrap();

    let window = video_subsystem.window("Chip 8 Emulator", 640, 320)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {
                    break 'running
                }

                Event::KeyDown { keycode, .. } => {
                    if let Some(key) = keycode {
                        if let Some(&index) = key_map.get(&key) {
                            input[index] = true;
                        }
                    }
                }
                Event::KeyUp { keycode, .. } => {
                    if let Some(key) = keycode {
                        if let Some(&index) = key_map.get(&key) {
                            input[index] = false;
                        }
                    }
                }
                _ => {}
            }
        }


        if delay_clock == speed / 60 {
            for (i,pixel) in graphics.iter().enumerate() {
                let x = ((i % 64) * 10) as i32;
                let y = (((i - (i % 64))  / 64)* 10) as i32;

                if *pixel == true {
                    canvas.set_draw_color(Color::RGB(255, 255, 255));
                    canvas.fill_rect(Rect::new(x,y , 10, 10)).unwrap()
                } else {
                    canvas.set_draw_color(Color::RGB(0, 0, 0));
                    canvas.fill_rect(Rect::new(x, y, 10,10)).unwrap()

                }
            }
            if delay_timer > 0 {delay_timer -= 1}
            if sound_timer > 0 {
                device.resume();
                sound_timer -= 1
            } else { device.pause(); }
            delay_clock = 1

        } else { delay_clock += 1; }



        cpu::run(&mut memory,&mut registers ,&mut stack, &mut graphics, &mut input, &mut delay_timer, &mut sound_timer, &mut pc, &mut sp, &mut index);

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / speed));
    }
}


