use rodio::{Decoder, OutputStream, Sink};
use std::error::Error;
use std::io::Cursor;
use std::thread;

pub fn play_audio(audio_data: Vec<u8>) -> Result<(), Box<dyn Error>> {
    thread::spawn(move || {
        // Create a Cursor from the audio data
        let audio_reader = Cursor::new(audio_data);

        // Load the audio data into a Decoder
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        let decoder = Decoder::new(audio_reader).unwrap();

        // Play the audio
        sink.append(decoder);
        sink.sleep_until_end();
    });

    Ok(())
}
