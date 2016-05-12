extern crate ears;
use ears::{Sound, AudioController};

fn main() {
    // Create a new Sound.
    let mut snd = match Sound::new("/usr/share/beepr/sounds/dingaling.ogg") {
        Some(val) => val,
        None => {
            println!("Could not open sound file at /usr/share/beepr/sounds/dingaling.ogg");
            return;
        }
    };

    // Play the Soun
    snd.play();

    // Wait until the end of the sound
    while snd.is_playing() {}
}
