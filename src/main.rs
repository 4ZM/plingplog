use rustysynth::{SoundFont, Synthesizer, SynthesizerSettings};
use std::fs::File;
use std::sync::Arc;
//use midly::Smf;
use rodio::{OutputStream, Source};

fn main() {
    // Load the SoundFont.
    let mut sf2 = File::open("/usr/share/sounds/sf2/default-GM.sf2").unwrap();
    let sound_font = Arc::new(SoundFont::new(&mut sf2).unwrap());

    // Create the synthesizer.
    let settings = SynthesizerSettings::new(44100);
    let mut synthesizer = Synthesizer::new(&sound_font, &settings).unwrap();

    // Play some notes (middle C, E, G).
    synthesizer.note_on(0, 60, 100);
    synthesizer.note_on(0, 64, 100);
    synthesizer.note_on(0, 67, 100);

    // The output buffer (3 seconds).
    let sample_count = (3 * settings.sample_rate) as usize;
    let mut left: Vec<f32> = vec![0_f32; sample_count];
    let mut right: Vec<f32> = vec![0_f32; sample_count];

    // Render the waveform.
    synthesizer.render(&mut left[..], &mut right[..]);

    // Set up audio output using rodio.
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let source = rodio::buffer::SamplesBuffer::new(2, settings.sample_rate as u32, left.clone());
    stream_handle.play_raw(source.convert_samples()).unwrap();

    // Sleep for 3 seconds to let the sound play.
    std::thread::sleep(std::time::Duration::from_secs(3));
}
