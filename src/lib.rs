#[cfg(target_os = "macos")]
mod macos;
#[cfg(not(any(target_os = "macos")))]
mod null;

pub trait MediaBackend {
    fn set_title(&self, title: &str);
    fn set_artist(&self, artist: &str);
    fn set_album(&self, album: &str);
    fn set_genre(&self, genre: &str);
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
    media_type: MediaType,
    duration: f64,
    playback_rate: f64,
}

pub struct MediaSession {
    backend: Box<dyn MediaBackend>,
    metadata: Metadata,
}

impl MediaSession {
    pub fn new() -> Self {
        #[cfg(target_os = "macos")]
        let backend = Box::new(macos::NowPlayingBackend::new());

        #[cfg(not(any(target_os = "macos")))]
        let backend = Box::new(null::NullBackend::new());

        Self {
            backend,
            metadata: Metadata {
                title: "".to_string(),
                artist: "".to_string(),
                album: "".to_string(),
                genre: "".to_string(),
                media_type: MediaType::Audio,
                duration: 0.0f64,
                playback_rate: 0.0f64,
            },
        }
    }

    pub fn set_title(&mut self, title: &str) {
        self.metadata.title = title.to_string();
        self.backend.set_title(title);
    }

    pub fn title(&self) -> &str {
        &self.metadata.title
    }

    pub fn set_artist(&mut self, artist: &str) {
        self.metadata.artist = artist.to_string();
        self.backend.set_artist(artist);
    }

    pub fn artist(&self) -> &str {
        &self.metadata.artist
    }

    pub fn set_album(&mut self, album: &str) {
        self.metadata.album = album.to_string();
        self.backend.set_album(album);
    }

    pub fn album(&self) -> &str {
        &self.metadata.album
    }

    pub fn set_genre(&mut self, genre: &str) {
        self.metadata.genre = genre.to_string();
        self.backend.set_album(genre);
    }

    pub fn genre(&self) -> &str {
        &self.metadata.genre
    }

    pub fn set_media_type(&mut self, media_type: MediaType) {
        self.metadata.media_type = media_type;
        self.backend.set_media_type(media_type);
    }

    pub fn media_type(&self) -> MediaType {
        self.metadata.media_type
    }

    pub fn set_playback_duration(&mut self, duration: f64) {
        self.metadata.duration = duration;
        self.backend.set_playback_duration(duration);
    }

    pub fn duration(&self) -> f64 {
        self.metadata.duration
    }

    pub fn set_elapsed_duration(&self, duration: f64) {
        self.backend.set_elapsed_duration(duration);
    }

    pub fn set_playback_rate(&mut self, rate: f64) {
        self.metadata.playback_rate = rate;
        self.backend.set_playback_rate(rate);
    }

    pub fn playback_rate(&self) -> f64 {
        self.metadata.playback_rate
    }

    pub fn start(&self) {
        self.backend.start_session();
    }

    pub fn stop(&self) {
        self.backend.stop_session();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
