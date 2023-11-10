extern crate sdl2;

use sdl2::audio::AudioSpecDesired;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use std::ops::{DerefMut, Deref};
use std::sync::mpsc;

mod callbacks;
mod net_client;
mod command;
mod utils;
mod voice_list_item;
mod voice_list_item_ui;
mod text_render;

use callbacks::{Recording, SoundPlayback};
use net_client::NetClient;
use text_render::TextRender;
use voice_list_item_ui::VoiceListItemUI;

#[derive(PartialEq)]
enum State {
    Idle,
    RecordStart,
    ReplayStart,
    Replaying
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

    let texturer = window_canvas.texture_creator();

    // FONT
    let ttf_context = sdl2::ttf::init().unwrap();
    let font = ttf_context.load_font("OpenSans-Light.ttf", 16).unwrap();
    let text_render = TextRender { font };
    // --------

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut state = State::Idle;
    let mut record_buffer: Vec<i16> = Vec::new();
    let mut records_list = NetClient::index().unwrap();
    let mut records_list_ui: Vec<VoiceListItemUI> = Vec::new();

    let mut list_y_offset = 0;
    for record in records_list.iter() {
        let item_surface = text_render.surface_from_timestamp(record.timestamp);
        let item_texture = texturer.create_texture_from_surface(&item_surface).unwrap();
        let mut item_rect = item_surface.rect();
        item_rect.y = list_y_offset;
        records_list_ui.push(
            VoiceListItemUI {
                timestamp: record.timestamp,
                rect: item_rect,
                texture: item_texture
            }
        );
        list_y_offset += item_rect.h;
    }

    window_canvas.set_draw_color(Color::RGB(250, 250, 245));

    let mut list_update_timer = utils::get_timestamp();

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
                        NetClient::post_record(&record_buffer);
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::P), .. } => {
                    let mut replay_lock = playback_device.lock();
                    let buffer = replay_lock.deref_mut();
                    buffer.data.clear();
                    buffer.data.extend(record_buffer.clone());
                    buffer.pos = 0;
                    if !buffer.is_empty() {
                        state = State::ReplayStart;
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                    // let download = NetClient::get_record().unwrap();

                    // let mut replay_lock = playback_device.lock();
                    // let buffer = replay_lock.deref_mut();
                    // buffer.data.clear();
                    // buffer.data.extend(download);
                    // buffer.pos = 0;
                    // if !buffer.is_empty() {
                    //     state = State::ReplayStart;
                    // }
                },
                Event::MouseWheel { y, .. } => {
                    let dy = y * 4;
                    for record_ui in records_list_ui.iter_mut() {
                        record_ui.rect.y += dy;
                    }
                },
                Event::MouseButtonDown { x, y, mouse_btn: MouseButton::Left, .. } => {
                    let record = records_list_ui.iter().find(|&item| {
                        item.rect.y >= 0 &&
                        (y > item.rect.y && y < item.rect.y + item.rect.h) &&
                        (x > item.rect.x && x < item.rect.x + item.rect.w)
                    });

                    if record.is_some() {
                        let ts = record.unwrap().timestamp;
                        println!("{ts:}");
                        let download = NetClient::get_record(ts).unwrap();

                        let mut replay_lock = playback_device.lock();
                        let buffer = replay_lock.deref_mut();
                        buffer.data.clear();
                        buffer.data.extend(download);
                        buffer.pos = 0;
                        if !buffer.is_empty() {
                            state = State::ReplayStart;
                        }
                    }
                }
                _ => {}
            }
        }

        if state == State::RecordStart {
            record_buffer.extend(done_receiver.recv().unwrap());
            println!("Record len = {:}", record_buffer.len());
        }

        if state == State::ReplayStart {
            println!("Replay");
            playback_device.resume();
            state = State::Replaying;
        }

        if state == State::Replaying {
            // scope the device lock guard
            {
                let replay_lock = playback_device.lock();
                let buffer = replay_lock.deref();
                if buffer.is_end_of_buffer() {
                    state = State::Idle;
                }
            }

            if state == State::Idle {
                playback_device.pause();
            }
        }

        // every 8 seconds update records list
        let ts = utils::get_timestamp();
        if ts - list_update_timer > 8 {
            records_list = NetClient::index().unwrap();
            list_update_timer = ts;
        }

        window_canvas.clear();
        for record_ui in records_list_ui.iter() {
            window_canvas.copy(&record_ui.texture, None, record_ui.rect).unwrap();
        }
        window_canvas.present();

    }

}
