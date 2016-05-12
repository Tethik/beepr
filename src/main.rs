extern crate ears;
use ears::{Sound, AudioController};

fn main() {
    // Create a new Sound.
    let mut snd: Sound = Sound::new("sounds/dingaling.ogg").unwrap();

    // Play the Soun
    snd.play();

    // Wait until the end of the sound
    while snd.is_playing() {}
}
