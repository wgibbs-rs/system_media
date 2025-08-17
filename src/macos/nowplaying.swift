import Foundation
import MediaPlayer
import AVFoundation

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
    let s = String(cString: title)
    var nowPlayingInfo = [String: Any]()
    nowPlayingInfo[MPMediaItemPropertyTitle] = s
    MPNowPlayingInfoCenter.default().nowPlayingInfo = nowPlayingInfo
}

@_cdecl("swift_set_metadata_artist")
public func setMetadataArtist(artist : UnsafePointer<CChar>) {
    let s = String(cString: artist)
    var nowPlayingInfo = [String: Any]()
    nowPlayingInfo[MPMediaItemPropertyArtist] = s
    MPNowPlayingInfoCenter.default().nowPlayingInfo = nowPlayingInfo
}

@_cdecl("swift_set_metadata_album")
public func setMetadataAlbumTitle(album : UnsafePointer<CChar>) {
    let s = String(cString: album)
    var nowPlayingInfo = [String: Any]()
    nowPlayingInfo[MPMediaItemPropertyAlbumTitle] = s
    MPNowPlayingInfoCenter.default().nowPlayingInfo = nowPlayingInfo
}

@_cdecl("swift_set_metadata_genre")
public func setMetadataGenre(genre : UnsafePointer<CChar>) {
    let s = String(cString: genre)
    var nowPlayingInfo = [String: Any]()
    nowPlayingInfo[MPMediaItemPropertyGenre] = s
    MPNowPlayingInfoCenter.default().nowPlayingInfo = nowPlayingInfo
}

@_cdecl("swift_set_metadata_media_type")
public func setMetadataMediaType(id: Int) {
    var nowPlayingInfo = [String: Any]()
    if (id == 0) {
        nowPlayingInfo[MPNowPlayingInfoPropertyMediaType] = MPNowPlayingInfoMediaType.audio.rawValue
    } else {
        nowPlayingInfo[MPNowPlayingInfoPropertyMediaType] = MPNowPlayingInfoMediaType.video.rawValue
    }
    MPNowPlayingInfoCenter.default().nowPlayingInfo = nowPlayingInfo
}

@_cdecl("swift_set_playback_duration")
public func setPlaybackDuration(seconds : Double) {
    var nowPlayingInfo = [String: Any]()
    nowPlayingInfo[MPMediaItemPropertyPlaybackDuration] = seconds
    MPNowPlayingInfoCenter.default().nowPlayingInfo = nowPlayingInfo
}

@_cdecl("swift_set_elapsed_duration")
public func setElapsedPlaybackTime(seconds : Double) {
    var nowPlayingInfo = [String: Any]()
    nowPlayingInfo[MPNowPlayingInfoPropertyElapsedPlaybackTime] = seconds
    MPNowPlayingInfoCenter.default().nowPlayingInfo = nowPlayingInfo
}

@_cdecl("swift_set_playback_rate")
public func setPlaybackRate(rate : Double) {
    var nowPlayingInfo = [String: Any]()
    nowPlayingInfo[MPNowPlayingInfoPropertyPlaybackRate] = rate
    MPNowPlayingInfoCenter.default().nowPlayingInfo = nowPlayingInfo
}

@_cdecl("swift_start_session")
public func startSession() {
    setupRemoteCommandTargets()     // Create event hooks for media session.
    setPlaybackRate(rate: 0.0)      // Ensures NowPlaying is properly displayed.
    RunLoop.main.run()              // Loop the thread indefinitely; until killed.
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