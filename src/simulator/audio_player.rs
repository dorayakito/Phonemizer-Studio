use rodio::{OutputStream, Sink, Source};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

#[derive(Clone)]
pub struct AudioTonePlayer {
    is_playing: Arc<AtomicBool>,
}

impl AudioTonePlayer {
    pub fn new() -> Self {
        Self {
            is_playing: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn is_playing(&self) -> bool {
        self.is_playing.load(Ordering::Relaxed)
    }

    pub fn stop(&self) {
        self.is_playing.store(false, Ordering::Relaxed);
    }

    pub fn play_phoneme_sequence(
        &self,
        notes_with_phonemes: Vec<(f32, u64)>, // (frequência Hz, duração ms)
    ) {
        let is_playing_flag = self.is_playing.clone();
        is_playing_flag.store(true, Ordering::Relaxed);

        std::thread::spawn(move || {
            let stream_res = OutputStream::try_default();
            if let Ok((_stream, handle)) = stream_res {
                if let Ok(sink) = Sink::try_new(&handle) {
                    for (freq, dur_ms) in notes_with_phonemes {
                        if !is_playing_flag.load(Ordering::Relaxed) {
                            break;
                        }

                        let duration = Duration::from_millis(dur_ms.max(20));
                        let source = rodio::source::SineWave::new(freq)
                            .take_duration(duration)
                            .amplify(0.20); // Volume suave

                        sink.append(source);
                    }
                    sink.sleep_until_end();
                }
            }
            is_playing_flag.store(false, Ordering::Relaxed);
        });
    }

    pub fn note_name_to_freq(note: &str) -> f32 {
        let note = note.trim().to_uppercase();
        if note.is_empty() || note == "R" || note == "REST" {
            return 0.0;
        }

        let base_freqs = [
            ("C", 261.63),
            ("C#", 277.18),
            ("DB", 277.18),
            ("D", 293.66),
            ("D#", 311.13),
            ("EB", 311.13),
            ("E", 329.63),
            ("F", 349.23),
            ("F#", 369.99),
            ("GB", 369.99),
            ("G", 392.00),
            ("G#", 415.30),
            ("AB", 415.30),
            ("A", 440.00),
            ("A#", 466.16),
            ("BB", 466.16),
            ("B", 493.88),
        ];

        let mut octave = 4;
        let mut key_part = note.clone();

        if let Some(last_char) = note.chars().last() {
            if last_char.is_ascii_digit() {
                octave = last_char.to_digit(10).unwrap_or(4) as i32;
                key_part = note[..note.len() - 1].to_string();
            }
        }

        for (k, f) in base_freqs {
            if k == key_part {
                let multiplier = 2.0f32.powi(octave - 4);
                return f * multiplier;
            }
        }

        440.0
    }
}
