extern crate sdl2;

use sdl2::audio::{AudioCallback, AudioSpecDesired};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::ops::DerefMut;
use std::sync::mpsc;

#[derive(PartialEq)]
enum State {
    Idle,
    RecordStart,
    Replay,
}

struct Recording {
    done_sender: mpsc::Sender<Vec<i16>>,
}

impl AudioCallback for Recording {
    type Channel = i16;

    fn callback(&mut self, input: &mut [i16]) {
        println!("Input len = {:}", input.len());
        self.done_sender
            .send(Vec::from(input))
            .expect("Could not send record buffer");
    }
}

struct SoundPlayback {
    data: Vec<i16>,
    pos: usize,
}

impl AudioCallback for SoundPlayback {
    type Channel = i16;

    fn callback(&mut self, out: &mut [i16]) {
        for dst in out.iter_mut() {
            *dst = *self.data.get(self.pos).unwrap_or(&0);
            self.pos += 1;
        }
    }
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let sdl_video = sdl_context.video().unwrap();
    let audio_subsystem = sdl_context.audio().unwrap();

    let (done_sender, done_receiver) = mpsc::channel();

    let capture_device = audio_subsystem.open_capture(
        None,
        &AudioSpecDesired { freq: Some(22050), channels: Some(1), samples: Some(1024) },
        |_spec| {
            Recording { done_sender }
    }).unwrap();

    let mut playback_device = audio_subsystem.open_playback(
        None,
        &AudioSpecDesired { freq: Some(22050), channels: Some(1), samples: Some(1024) },
        |_spec| {
            SoundPlayback {
                data: Vec::new(),
                pos: 0,
            }
    }).unwrap();

    let window = sdl_video
        .window("Eremeev: Voice Chat", 400, 200)
        .position_centered()
        .build()
        .unwrap();

    let mut window_canvas = window.into_canvas()
        .present_vsync()
        .build().unwrap();

    window_canvas.set_draw_color(Color::RGB(250, 250, 245));
    window_canvas.clear();
    window_canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut state = State::Idle;
    let mut record_buffer: Vec<i16> = Vec::new();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                    if state == State::Idle {
                        state = State::RecordStart;
                        record_buffer.clear();
                        capture_device.resume();
                    }
                },
                Event::KeyUp { keycode: Some(Keycode::R), .. } => {
                    if state == State::RecordStart {
                        state = State::Idle;
                        capture_device.pause();
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::P), .. } => {
                    let mut replay_lock = playback_device.lock();
                    let buffer = replay_lock.deref_mut();
                    buffer.data.clear();
                    buffer.data.extend(record_buffer.clone());
                    buffer.pos = 0;
                    state = State::Replay;
                },
                _ => {}
            }
        }

        if state == State::RecordStart {
            record_buffer.extend(done_receiver.recv().unwrap());
            println!("Record len = {:}", record_buffer.len());
        }

        if state == State::Replay {
            println!("RGOGOG");
            state = State::Idle;
            playback_device.resume();
        }

        window_canvas.clear();
        window_canvas.present();
    }

}
