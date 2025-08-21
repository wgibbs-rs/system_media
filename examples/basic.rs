use system_media::MediaSession;
use std::fs;
use std::thread;
use std::time::Duration;

fn main() {
    println!("Now Playing session started");
    println!("Keeping process alive... Press Ctrl+C to exit");
    let mut session = MediaSession::new();

    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let path = format!("{}/examples/bug.png", manifest_dir);

    session.set_playback_rate(1.0);
    session.set_playback_duration(300.0);
    session.set_elapsed_duration(100.0);

    session.set_title("Help ME!");

    session.set_image(&path);

    session.start();
}
