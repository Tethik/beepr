extern crate ears;
use ears::{Sound, AudioController};

include!(concat!(env!("OUT_DIR"), "/build_config.rs"));

fn main() {
    let sound_file = sound_directory().to_string() + "sounds/dingaling.ogg";
    // Create a new Sound.
    let mut snd = match Sound::new(&*sound_file) {
        Some(val) => val,
        None => {
            println!("Could not open sound file at {}", sound_file);
            return;
        }
    };

    // Play the Soun
    snd.play();

    // Wait until the end of the sound
    while snd.is_playing() {}
}
