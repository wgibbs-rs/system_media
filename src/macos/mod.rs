use crate::{MediaBackend, MediaType};
use std::ffi::CString;

// Swift functions called by Rust
unsafe extern "C" {
    fn swift_set_metadata_title(title: *mut i8);
    fn swift_set_metadata_artist(artist: *mut i8);
    fn swift_set_metadata_album(album: *mut i8);
    fn swift_set_metadata_genre(genre: *mut i8);
    fn swift_set_metadata_media_type(id: i64);
    fn swift_set_playback_duration(seconds: f64);
    fn swift_set_elapsed_duration(seconds: f64);
    fn swift_set_playback_rate(rate: f64);
    fn swift_start_session();
}

// Rust functions called by Swift
// TODO: Add hooks for SystemMedia events.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_resume_playback_command() {}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_pause_playback_command() {}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_next_track_playback_command() {}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_previous_track_playback_command() {}

fn str_to_raw(s: &str) -> *mut i8 {
    let c_string = CString::new(s).expect("CString::new() failed. ");
    c_string.into_raw()
}

pub struct NowPlayingBackend;

impl NowPlayingBackend {
    pub fn new() -> Self {
        Self
    }
}

impl MediaBackend for NowPlayingBackend {
    fn set_title(&self, title: &str) {
        unsafe {
            swift_set_metadata_title(str_to_raw(title));
        }
    }

    fn set_artist(&self, artist: &str) {
        unsafe {
            swift_set_metadata_artist(str_to_raw(artist));
        }
    }

    fn set_album(&self, album: &str) {
        unsafe {
            swift_set_metadata_album(str_to_raw(album));
        }
    }

    fn set_genre(&self, genre: &str) {
        unsafe {
            swift_set_metadata_genre(str_to_raw(genre));
        }
    }

    fn set_media_type(&self, media_type: MediaType) {
        unsafe {
            if media_type == MediaType::Audio {
                swift_set_metadata_media_type(0);
            } else {
                swift_set_metadata_media_type(1);
            }
        }
    }

    fn set_playback_duration(&self, duration: f64) {
        unsafe {
            swift_set_playback_duration(duration);
        }
    }

    fn set_elapsed_duration(&self, duration: f64) {
        unsafe {
            swift_set_elapsed_duration(duration);
        }
    }

    fn set_playback_rate(&self, rate: f64) {
        unsafe {
            swift_set_playback_rate(rate);
        }
    }

    fn start_session(&self) {
        unsafe {
            swift_start_session();
        }
    }

    fn stop_session(&self) {
        println!("How do we stop a session?");
    }
}
