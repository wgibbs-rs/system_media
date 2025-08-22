use crate::{MediaBackend, MediaType};
use std::error::Error;

pub struct NullBackend;

impl NullBackend {
    pub fn new() -> Self {
        Self
    }
}

impl MediaBackend for NullBackend {
    fn set_title(&self, title: &str) {
        unimplemented!("system-media currently only supports macOS.");
    }

    fn set_artist(&self, artist: &str) {
        unimplemented!("system-media currently only supports macOS.");
    }

    fn set_album(&self, album: &str) {
        unimplemented!("system-media currently only supports macOS.");
    }

    fn set_genre(&self, genre: &str) {
        unimplemented!("system-media currently only supports macOS.");
    }

    fn set_image(&self, path: &str) -> Result<(), Box<dyn Error>> {
        unimplemented!("system-media currently only supports macOS.")
    }

    fn set_media_type(&self, media_type: MediaType) {
        unimplemented!("system-media currently only supports macOS.");
    }

    fn set_playback_duration(&self, duration: f64) {
        unimplemented!("system-media currently only supports macOS.");
    }

    fn set_elapsed_duration(&self, duration: f64) {
        unimplemented!("system-media currently only supports macOS.");
    }

    fn set_playback_rate(&self, rate: f64) {
        unimplemented!("system-media currently only supports macOS.");
    }

    fn start_session(&self) {
        unimplemented!("system-media currently only supports macOS.");
    }

    fn stop_session(&self) {
        unimplemented!("system-media currently only supports macOS.");
        println!("how do we stop a session?");
    }
}
