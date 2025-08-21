use std::thread;
use std::time::Duration;
use system_media::MediaSession;

fn main() {
    println!("Now Playing session started");
    println!("Keeping process alive... Press Ctrl+C to exit");
    let mut session = MediaSession::new();

    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let path = format!("{}/examples/RustAudio.png", manifest_dir);

    session.set_playback_rate(1.0);
    session.set_playback_duration(300.0);
    session.set_elapsed_duration(100.0);

    session.set_title("Example Title");
    session.set_artist("RustAudio");
    session.set_album("Audio and Video");

    session.set_image(&path).unwrap();

    session.start();
}
