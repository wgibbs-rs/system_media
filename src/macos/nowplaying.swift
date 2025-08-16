import Foundation
import MediaPlayer
import AVFoundation

// Tell Swift this function exists somewhere at runtime
@_silgen_name("rust_hello")
public func rustHello()

@_cdecl("hello_world")
public func startNowPlayingSession() {
    // Get the shared MPNowPlayingInfoCenter
    let nowPlayingInfoCenter = MPNowPlayingInfoCenter.default()
    
    // Set up remote command targets BEFORE setting the info
    setupRemoteCommandTargets()
    
    // Create now playing info dictionary
    var nowPlayingInfo = [String: Any]()
    
    // Set basic metadata
    nowPlayingInfo[MPMediaItemPropertyTitle] = "Your Song Title"
    nowPlayingInfo[MPMediaItemPropertyArtist] = "Your Artist Name" 
    nowPlayingInfo[MPMediaItemPropertyAlbumTitle] = "Your Album Name"
    nowPlayingInfo[MPMediaItemPropertyPlaybackDuration] = 180.0
    nowPlayingInfo[MPNowPlayingInfoPropertyElapsedPlaybackTime] = 0.0
    // This is crucial - set to 1.0 to indicate "playing" state
    nowPlayingInfo[MPNowPlayingInfoPropertyPlaybackRate] = 1.0
    // Add media type
    nowPlayingInfo[MPNowPlayingInfoPropertyMediaType] = MPNowPlayingInfoMediaType.audio.rawValue
    
    // Set the now playing info
    nowPlayingInfoCenter.nowPlayingInfo = nowPlayingInfo
    
    print("Now Playing session started")
    print("NowPlaying info set: \(nowPlayingInfo)")
    
    // Keep the process alive so the Now Playing info persists
    print("Keeping process alive... Press Ctrl+C to exit")
    RunLoop.main.run()
}

private func setupRemoteCommandTargets() {
    let commandCenter = MPRemoteCommandCenter.shared()
    
    // Play command
    commandCenter.playCommand.addTarget { event in
        // Handle play action
        print("Play command received")
        return .success
    }
    
    // Pause command
    commandCenter.pauseCommand.addTarget { event in
        // Handle pause action
        print("Pause command received")
        return .success
    }
    
    // Next track command
    commandCenter.nextTrackCommand.addTarget { event in
        // Handle next track action
        print("Next track command received")
        return .success
    }
    
    // Previous track command
    commandCenter.previousTrackCommand.addTarget { event in
        // Handle previous track action
        print("Previous track command received")
        return .success
    }
}