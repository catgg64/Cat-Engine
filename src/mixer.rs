use std::{collections::HashMap, fs::File, io::BufReader};
use rodio::{OutputStream, OutputStreamBuilder, Sink, Decoder};

pub struct AudioManager {
    stream: rodio::OutputStream,
    pub sounds: HashMap<u32, Sink>,
    next_id: u32,
}

impl AudioManager {
    pub fn new() -> Self {
        let stream = OutputStreamBuilder::open_default_stream().unwrap();

        Self {
            stream,
            sounds: HashMap::new(),
            next_id: 0,
        }
    }

    pub fn play_sound(&mut self, path: &str) -> u32 {
        let file = std::fs::File::open(path).unwrap();
        let source = rodio::Decoder::try_from(file).unwrap();

        let sink = Sink::connect_new(self.stream.mixer());
        sink.append(source);

        let id = self.next_id;
        self.next_id += 1;

        self.sounds.insert(id, sink);

        id
    }

    pub fn stop_sound(&mut self, id: u32) {
        if let Some(sink) = self.sounds.remove(&id) {
            sink.stop();
        }
    }

    pub fn shutdown(&mut self) {
        for sink in self.sounds.values() {
            sink.stop();
        }

        self.sounds.clear();
    }
}