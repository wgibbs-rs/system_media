#[cfg(target_os = "macos")]
mod macos;
#[cfg(not(any(target_os = "macos")))]
mod null;

use std::sync::{Arc, Mutex};
use std::error::Error;
use image::ImageReader;

pub trait MediaBackend {
    fn set_title(&self, title: &str);
    fn set_artist(&self, artist: &str);
    fn set_album(&self, album: &str);
    fn set_genre(&self, genre: &str);
    fn set_image(&self, path: &str);
    fn set_media_type(&self, media_type: MediaType);
    fn set_playback_duration(&self, duration: f64);
    fn set_elapsed_duration(&self, duration: f64);
    fn set_playback_rate(&self, rate: f64);
    fn start_session(&self);
    fn stop_session(&self);
}

// Defines the type of media being presented
#[derive(Clone, Copy, PartialEq)]
pub enum MediaType {
    Audio,
    Video,
}

pub struct Metadata {
    title: String,
    artist: String,
    album: String,
    genre: String,
    image_path: String,
    media_type: MediaType,
    duration: f64,
    playback_rate: f64,
}

fn decode_image(path: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let img = ImageReader::open(path)?
        .decode()?
        .to_rgb8();
    Ok(img.into_raw())
}

pub struct MediaSession {
    backend: Box<dyn MediaBackend>,
    metadata: Arc<Mutex<Metadata>>,
}

impl MediaSession {
    pub fn new() -> Self {
        #[cfg(target_os = "macos")]
        let backend = Box::new(macos::NowPlayingBackend::new());

        #[cfg(not(any(target_os = "macos")))]
        let backend = Box::new(null::NullBackend::new());

        Self {
            backend,
            metadata: Arc::new(Mutex::new(Metadata {
                title: "".to_string(),
                artist: "".to_string(),
                album: "".to_string(),
                genre: "".to_string(),
                image_path: "".to_string(),
                media_type: MediaType::Audio,
                duration: 0.0,
                playback_rate: 0.0,
            })),
        }
    }

    pub fn set_title(&mut self, title: &str) {
        Arc::clone(&self.metadata).lock().unwrap().title = title.to_string();
        self.backend.set_title(title);
    }

    pub fn title(&self) -> String {
        let data = self.metadata.lock().unwrap();
        data.title.clone()
    }

    pub fn set_artist(&mut self, artist: &str) {
        Arc::clone(&self.metadata).lock().unwrap().artist = artist.to_string();
        self.backend.set_artist(artist);
    }

    pub fn artist(&self) -> String {
        let data = self.metadata.lock().unwrap();
        data.artist.clone()
    }

    pub fn set_album(&mut self, album: &str) {
        Arc::clone(&self.metadata).lock().unwrap().album = album.to_string();
        self.backend.set_album(album);
    }

    pub fn album(&self) -> String {
        let data = self.metadata.lock().unwrap();
        data.album.clone()
    }

    pub fn set_genre(&mut self, genre: &str) {
        Arc::clone(&self.metadata).lock().unwrap().genre = genre.to_string();
        self.backend.set_album(genre);
    }

    pub fn genre(&self) -> String {
        let md = self.metadata.lock().unwrap();
        md.genre.clone()
    }

    pub fn set_image(&mut self, path: &str) -> Result<(), Box<dyn Error>> {
        Arc::clone(&self.metadata).lock().unwrap().image_path = path.to_string();
        self.backend.set_image(path);
        Ok(())
    }

    pub fn image(&self) -> String {
        let md = self.metadata.lock().unwrap();
        md.image_path.clone()
    }

    pub fn set_media_type(&mut self, media_type: MediaType) {
        Arc::clone(&self.metadata).lock().unwrap().media_type = media_type;
        self.backend.set_media_type(media_type);
    }

    pub fn media_type(&self) -> MediaType {
        Arc::clone(&self.metadata).lock().unwrap().media_type
    }

    pub fn set_playback_duration(&mut self, duration: f64) {
        Arc::clone(&self.metadata).lock().unwrap().duration = duration;
        self.backend.set_playback_duration(duration);
    }

    pub fn duration(&self) -> f64 {
        Arc::clone(&self.metadata).lock().unwrap().duration
    }

    pub fn set_elapsed_duration(&self, duration: f64) {
        self.backend.set_elapsed_duration(duration);
    }

    pub fn set_playback_rate(&mut self, rate: f64) {
        Arc::clone(&self.metadata).lock().unwrap().playback_rate = rate;
        self.backend.set_playback_rate(rate);
    }

    pub fn playback_rate(&self) -> f64 {
        Arc::clone(&self.metadata).lock().unwrap().playback_rate
    }

    pub fn start(&self) {
        self.backend.start_session();
    }

    pub fn stop(&self) {
        self.backend.stop_session();
    }
}
