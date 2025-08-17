use system_media::MediaSession;

fn main() {
    println!("Now Playing session started");
    println!("Keeping process alive... Press Ctrl+C to exit");
    let mut session = MediaSession::new();
    session.set_title("Hello!");
    session.start();
}
