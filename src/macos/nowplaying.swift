import Foundation
import MediaPlayer
import AVFoundation
import AppKit
import Cocoa

@_silgen_name("rust_resume_playback_command")
public func rustStartPlaybackCommand()

@_silgen_name("rust_pause_playback_command")
public func rustPausePlaybackCommand()

@_silgen_name("rust_next_track_playback_command")
public func rustNextTrackCommand()

@_silgen_name("rust_previous_track_playback_command")
public func rustPreviousTrackCommand()

@_cdecl("swift_set_metadata_title")
public func setMetadataTitle(title : UnsafePointer<CChar>) {
    DispatchQueue.main.async {
        let s = String(cString: title)
        var nowPlayingInfo = MPNowPlayingInfoCenter.default().nowPlayingInfo ?? [String: Any]()
        nowPlayingInfo[MPMediaItemPropertyTitle] = s
        MPNowPlayingInfoCenter.default().nowPlayingInfo = nowPlayingInfo
    }
}

@_cdecl("swift_set_metadata_artist")
public func setMetadataArtist(artist : UnsafePointer<CChar>) {
    DispatchQueue.main.async {
        let s = String(cString: artist)
        var nowPlayingInfo = MPNowPlayingInfoCenter.default().nowPlayingInfo ?? [String: Any]()
        nowPlayingInfo[MPMediaItemPropertyArtist] = s
        MPNowPlayingInfoCenter.default().nowPlayingInfo = nowPlayingInfo
    }
}

@_cdecl("swift_set_metadata_album")
public func setMetadataAlbumTitle(album : UnsafePointer<CChar>) {
    DispatchQueue.main.async {
        let s = String(cString: album)
        var nowPlayingInfo = MPNowPlayingInfoCenter.default().nowPlayingInfo ?? [String: Any]()
        nowPlayingInfo[MPMediaItemPropertyAlbumTitle] = s
        MPNowPlayingInfoCenter.default().nowPlayingInfo = nowPlayingInfo
    }
}

@_cdecl("swift_set_metadata_genre")
public func setMetadataGenre(genre : UnsafePointer<CChar>) {
    DispatchQueue.main.async {
        let s = String(cString: genre)
        var nowPlayingInfo = MPNowPlayingInfoCenter.default().nowPlayingInfo ?? [String: Any]()
        nowPlayingInfo[MPMediaItemPropertyGenre] = s
        MPNowPlayingInfoCenter.default().nowPlayingInfo = nowPlayingInfo
    }
}

func resize(image: NSImage, to newSize: NSSize) -> NSImage {
    let resizedImage = NSImage(size: newSize)
    resizedImage.lockFocus()
    defer { resizedImage.unlockFocus() }

    image.draw(
        in: NSRect(origin: .zero, size: newSize),
        from: NSRect(origin: .zero, size: image.size),
        operation: .copy,
        fraction: 1.0
    )

    return resizedImage
}

@_cdecl("swift_set_metadata_image")
public func setMetadataImage(bytes: UnsafePointer<UInt8>, length: Int) {
    DispatchQueue.main.async {
        let data = Data(bytes: bytes, count: length)
        guard let image = NSImage(data: data) else { return }
        
        let artwork = MPMediaItemArtwork(boundsSize: image.size) { size in
            let newImage = NSImage(size: size)
            newImage.lockFocus()
            image.draw(in: NSRect(origin: .zero, size: size))
            newImage.unlockFocus()
            return newImage
        }
        
        var info = MPNowPlayingInfoCenter.default().nowPlayingInfo ?? [:]
        info[MPMediaItemPropertyArtwork] = artwork
        MPNowPlayingInfoCenter.default().nowPlayingInfo = info
    }
}

@_cdecl("swift_set_metadata_media_type")
public func setMetadataMediaType(id: Int) {
    DispatchQueue.main.async {
        var nowPlayingInfo = MPNowPlayingInfoCenter.default().nowPlayingInfo ?? [String: Any]()
        if (id == 0) {
            nowPlayingInfo[MPNowPlayingInfoPropertyMediaType] = MPNowPlayingInfoMediaType.audio.rawValue
        } else {
            nowPlayingInfo[MPNowPlayingInfoPropertyMediaType] = MPNowPlayingInfoMediaType.video.rawValue
        }
        MPNowPlayingInfoCenter.default().nowPlayingInfo = nowPlayingInfo
    }
}

@_cdecl("swift_set_playback_duration")
public func setPlaybackDuration(seconds : Double) {
    DispatchQueue.main.async {
        var nowPlayingInfo = MPNowPlayingInfoCenter.default().nowPlayingInfo ?? [String: Any]()
        nowPlayingInfo[MPMediaItemPropertyPlaybackDuration] = seconds
        MPNowPlayingInfoCenter.default().nowPlayingInfo = nowPlayingInfo
    }
}

@_cdecl("swift_set_elapsed_duration")
public func setElapsedPlaybackTime(seconds : Double) {
    DispatchQueue.main.async {
        var nowPlayingInfo = MPNowPlayingInfoCenter.default().nowPlayingInfo ?? [String: Any]()
        nowPlayingInfo[MPNowPlayingInfoPropertyElapsedPlaybackTime] = seconds
        MPNowPlayingInfoCenter.default().nowPlayingInfo = nowPlayingInfo
    }
}

@_cdecl("swift_set_playback_rate")
public func setPlaybackRate(rate : Double) {
    DispatchQueue.main.async {
        var nowPlayingInfo = MPNowPlayingInfoCenter.default().nowPlayingInfo ?? [String: Any]()
        nowPlayingInfo[MPNowPlayingInfoPropertyPlaybackRate] = rate
        MPNowPlayingInfoCenter.default().nowPlayingInfo = nowPlayingInfo
    }
}

@_cdecl("swift_start_session")
public func startSession() {
    setupRemoteCommandTargets()     // Create event hooks for media session.
    RunLoop.main.run()              // Loop the thread indefinitely until killed.
}

private func setupRemoteCommandTargets() {
    let commandCenter = MPRemoteCommandCenter.shared()
    
    commandCenter.playCommand.addTarget { event in
        rustStartPlaybackCommand()
        return .success
    }
    
    commandCenter.pauseCommand.addTarget { event in
        rustPausePlaybackCommand()
        return .success
    }
    
    commandCenter.nextTrackCommand.addTarget { event in
        rustNextTrackCommand()
        return .success
    }
    
    commandCenter.previousTrackCommand.addTarget { event in
        rustPreviousTrackCommand()
        return .success
    }
}