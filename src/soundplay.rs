use std::time::Duration;
use rodio::{OutputStream, Sink};
use rodio::source::{SineWave, Source};
use std::error::Error;
use rand::Rng;

pub fn play_audio(index: usize) -> Result<(), Box<dyn Error>> {
    let notes: [f32; 63] = [17.32, 18.35, 20.60, 23.12, 25.96, 27.50, 30.87,
                            34.65, 36.71, 41.20, 46.25, 51.91, 55.00, 61.74,
                            69.30, 73.42, 82.41, 92.50, 103.83, 110.00, 123.47,
                            138.59, 146.83, 164.81, 185.00, 207.65, 220.00, 246.94,
                            277.18, 293.66, 329.63, 369.99, 415.30, 440.00, 493.88,
                            554.37, 587.33, 659.25, 739.99, 830.61, 880.00, 987.77,
                            1108.73, 1174.66, 1318.51, 1479.98, 1661.22, 1760.00, 1975.53,
                            2217.46, 2349.32, 2637.02, 2959.96, 3322.44, 3520.00, 3951.07,
                            4434.92, 4698.63, 5274.04, 5919.91, 6644.88, 7040.00, 7902.13];

    if index >= notes.len() {
        return Err("Invalid index".into());
    }
                        
    let frequency = notes[index];
    let mut rng = rand::thread_rng();
    let random_float: f32 = rng.gen_range(0.1..2.0);

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    let source = SineWave::new(frequency).take_duration(Duration::from_secs_f32(random_float)).amplify(1.0);
    sink.append(source);
                        
    print!("Frequency: {}, ", frequency);
    sink.sleep_until_end();
                        
    Ok(())
}
