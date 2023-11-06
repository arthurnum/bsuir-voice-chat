use sdl2::audio::AudioCallback;
use std::sync::mpsc;

pub struct Recording {
    pub done_sender: mpsc::Sender<Vec<i16>>,
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

pub struct SoundPlayback {
    pub data: Vec<i16>,
    pub pos: usize,
}

impl SoundPlayback {
    pub fn is_end_of_buffer(&self) -> bool {
        return self.pos >= self.data.len();
    }

    pub fn is_empty(&self) -> bool {
        return self.data.is_empty();
    }
}

impl AudioCallback for SoundPlayback {
    type Channel = i16;

    fn callback(&mut self, out: &mut [i16]) {
        println!("OUTPUT len = {:}", out.len());
        let buf_slice = self.data.as_slice();
        out.clone_from_slice(
            &buf_slice[(self.pos)..(self.pos + 1024)]
        );
        self.pos += 1024;
    }
}
