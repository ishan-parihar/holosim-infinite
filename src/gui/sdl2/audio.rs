//! SDL2 Audio Module
//!
//! Provides SDL2_mixer-based audio support for the simulation.
//! Infrastructure is future-proofed for emergence sounds and notifications.
//!
//! From SDL2_INTEGRATION_ROADMAP.md:
//! "Audio not immediately used, but infrastructure ready for future features (emergence sounds, notifications)"

use std::collections::HashMap;
use std::path::Path;
use std::sync::Mutex;

use sdl2::mixer::{
    Chunk, Music, AUDIO_F32, AUDIO_S16LSB, AUDIO_S16MSB, AUDIO_S8, AUDIO_U16LSB, AUDIO_U16MSB,
    AUDIO_U8, DEFAULT_CHANNELS, DEFAULT_FREQUENCY,
};

/// Audio error types
#[derive(Debug, Clone, PartialEq)]
pub enum AudioError {
    /// SDL2_mixer initialization failed
    InitializationFailed(String),
    /// Sound file not found
    SoundNotFound(String),
    /// Sound loading failed
    SoundLoadFailed(String),
    /// Playback failed
    PlaybackFailed(String),
    /// Invalid sound ID
    InvalidSoundId(u32),
    /// Music file not found
    MusicNotFound(String),
    /// Music loading failed
    MusicLoadFailed(String),
    /// Music playback failed
    MusicPlaybackFailed(String),
    /// Volume out of range (0-128)
    InvalidVolume(u32),
    /// Audio system not initialized
    NotInitialized,
}

impl std::fmt::Display for AudioError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AudioError::InitializationFailed(msg) => {
                write!(f, "Audio initialization failed: {}", msg)
            }
            AudioError::SoundNotFound(path) => write!(f, "Sound not found: {}", path),
            AudioError::SoundLoadFailed(msg) => write!(f, "Sound load failed: {}", msg),
            AudioError::PlaybackFailed(msg) => write!(f, "Playback failed: {}", msg),
            AudioError::InvalidSoundId(id) => write!(f, "Invalid sound ID: {}", id),
            AudioError::MusicNotFound(path) => write!(f, "Music not found: {}", path),
            AudioError::MusicLoadFailed(msg) => write!(f, "Music load failed: {}", msg),
            AudioError::MusicPlaybackFailed(msg) => write!(f, "Music playback failed: {}", msg),
            AudioError::InvalidVolume(vol) => write!(f, "Invalid volume: {} (must be 0-128)", vol),
            AudioError::NotInitialized => write!(f, "Audio system not initialized"),
        }
    }
}

impl std::error::Error for AudioError {}

/// Audio format specification
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AudioFormat {
    /// 8-bit unsigned
    U8,
    /// 8-bit signed
    S8,
    /// 16-bit unsigned little-endian
    U16LSB,
    /// 16-bit unsigned big-endian
    U16MSB,
    /// 16-bit signed little-endian
    S16LSB,
    /// 16-bit signed big-endian
    S16MSB,
    /// 32-bit floating point
    F32,
}

impl AudioFormat {
    /// Convert to SDL2_mixer format
    fn to_sdl2_format(&self) -> u16 {
        match self {
            AudioFormat::U8 => AUDIO_U8,
            AudioFormat::S8 => AUDIO_S8,
            AudioFormat::U16LSB => AUDIO_U16LSB,
            AudioFormat::U16MSB => AUDIO_U16MSB,
            AudioFormat::S16LSB => AUDIO_S16LSB,
            AudioFormat::S16MSB => AUDIO_S16MSB,
            AudioFormat::F32 => AUDIO_F32,
        }
    }
}

/// Audio configuration
#[derive(Debug, Clone)]
pub struct AudioConfig {
    /// Sample frequency (Hz)
    pub frequency: i32,
    /// Audio format
    pub format: AudioFormat,
    /// Number of channels (1 = mono, 2 = stereo)
    pub channels: i32,
    /// Number of simultaneous channels for sound effects
    pub chunk_size: i32,
}

impl Default for AudioConfig {
    fn default() -> Self {
        AudioConfig {
            frequency: DEFAULT_FREQUENCY,
            format: AudioFormat::S16LSB,
            channels: DEFAULT_CHANNELS,
            chunk_size: 2048,
        }
    }
}

impl AudioConfig {
    /// Create new audio config with defaults
    pub fn new() -> Self {
        Self::default()
    }

    /// Set frequency
    pub fn with_frequency(mut self, freq: i32) -> Self {
        self.frequency = freq;
        self
    }

    /// Set format
    pub fn with_format(mut self, format: AudioFormat) -> Self {
        self.format = format;
        self
    }

    /// Set channels
    pub fn with_channels(mut self, channels: i32) -> Self {
        self.channels = channels;
        self
    }

    /// Set chunk size
    pub fn with_chunk_size(mut self, chunk_size: i32) -> Self {
        self.chunk_size = chunk_size;
        self
    }
}

/// Sound effect identifier
pub type SoundId = u32;

/// Audio manager for handling sound effects and music
pub struct AudioManager {
    /// Sound effects library
    sounds: HashMap<SoundId, Chunk>,
    /// Next available sound ID
    next_sound_id: SoundId,
    /// Currently loaded music
    current_music: Option<Music<'static>>,
    /// Sound effect volume (0-128)
    sfx_volume: i32,
    /// Music volume (0-128)
    music_volume: i32,
    /// Number of channels allocated for sound effects
    num_channels: i32,
}

impl AudioManager {
    /// Initialize audio manager with default configuration
    pub fn new() -> Result<Self, AudioError> {
        Self::with_config(&AudioConfig::default())
    }

    /// Initialize audio manager with custom configuration
    pub fn with_config(config: &AudioConfig) -> Result<Self, AudioError> {
        // Initialize SDL2_mixer
        let frequency = config.frequency;
        let format = config.format.to_sdl2_format();
        let channels = config.channels;
        let chunk_size = config.chunk_size;

        sdl2::mixer::open_audio(frequency, format, channels, chunk_size)
            .map_err(|e| AudioError::InitializationFailed(e.to_string()))?;

        // Allocate channels for sound effects (default: 8)
        let num_channels = 8;
        sdl2::mixer::allocate_channels(num_channels);

        Ok(AudioManager {
            sounds: HashMap::new(),
            next_sound_id: 1,
            current_music: None,
            sfx_volume: 128,
            music_volume: 128,
            num_channels,
        })
    }

    /// Load a sound effect from file
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the sound file (WAV, OGG, MP3 supported)
    ///
    /// # Returns
    ///
    /// Sound ID for the loaded sound
    pub fn load_sound<P: AsRef<Path>>(&mut self, path: P) -> Result<SoundId, AudioError> {
        let path_str = path.as_ref().to_string_lossy().to_string();

        if !path.as_ref().exists() {
            return Err(AudioError::SoundNotFound(path_str));
        }

        let chunk = Chunk::from_file(path.as_ref())
            .map_err(|e| AudioError::SoundLoadFailed(format!("{}: {}", path_str, e)))?;

        let sound_id = self.next_sound_id;
        self.sounds.insert(sound_id, chunk);
        self.next_sound_id += 1;

        Ok(sound_id)
    }

    /// Play a sound effect
    ///
    /// # Arguments
    ///
    /// * `sound_id` - ID of the sound to play
    /// * `_channel` - Channel to play on (-1 for first available, currently unused)
    /// * `loops` - Number of times to loop (-1 for infinite, 0 for once)
    pub fn play_sound(
        &self,
        sound_id: SoundId,
        _channel: i32,
        loops: i32,
    ) -> Result<(), AudioError> {
        let chunk = self
            .sounds
            .get(&sound_id)
            .ok_or(AudioError::InvalidSoundId(sound_id))?;

        sdl2::mixer::Channel::all()
            .play(chunk, loops)
            .map_err(|e| AudioError::PlaybackFailed(e.to_string()))?;

        Ok(())
    }

    /// Play a sound effect on a specific channel
    ///
    /// # Arguments
    ///
    /// * `sound_id` - ID of the sound to play
    /// * `channel` - Specific channel to play on (0-7)
    /// * `loops` - Number of times to loop (-1 for infinite, 0 for once)
    pub fn play_sound_on_channel(
        &self,
        sound_id: SoundId,
        channel: i32,
        loops: i32,
    ) -> Result<(), AudioError> {
        if channel < 0 || channel >= self.num_channels {
            return Err(AudioError::PlaybackFailed(format!(
                "Invalid channel: {} (must be 0-{})",
                channel,
                self.num_channels - 1
            )));
        }

        let chunk = self
            .sounds
            .get(&sound_id)
            .ok_or(AudioError::InvalidSoundId(sound_id))?;

        sdl2::mixer::Channel(channel)
            .play(chunk, loops)
            .map_err(|e| AudioError::PlaybackFailed(e.to_string()))?;

        Ok(())
    }

    /// Stop all sound effects
    pub fn stop_all_sounds(&self) {
        sdl2::mixer::Channel::all().halt();
    }

    /// Stop sound on a specific channel
    pub fn stop_channel(&self, channel: i32) {
        if channel >= 0 && channel < self.num_channels {
            sdl2::mixer::Channel(channel).halt();
        }
    }

    /// Check if a channel is playing
    pub fn is_channel_playing(&self, channel: i32) -> bool {
        if channel >= 0 && channel < self.num_channels {
            sdl2::mixer::Channel(channel).is_playing()
        } else {
            false
        }
    }

    /// Load and play background music
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the music file
    /// * `loops` - Number of times to loop (-1 for infinite, 0 for once)
    pub fn play_music<P: AsRef<Path>>(&mut self, path: P, loops: i32) -> Result<(), AudioError> {
        let path_str = path.as_ref().to_string_lossy().to_string();

        if !path.as_ref().exists() {
            return Err(AudioError::MusicNotFound(path_str));
        }

        // Load music
        let music = Music::from_file(path.as_ref())
            .map_err(|e| AudioError::MusicLoadFailed(format!("{}: {}", path_str, e)))?;

        // Play music
        music
            .play(loops)
            .map_err(|e| AudioError::MusicPlaybackFailed(e.to_string()))?;

        self.current_music = Some(music);
        Ok(())
    }

    /// Stop background music
    pub fn stop_music(&self) {
        sdl2::mixer::Music::halt();
    }

    /// Pause background music
    pub fn pause_music(&self) {
        sdl2::mixer::Music::pause();
    }

    /// Resume background music
    pub fn resume_music(&self) {
        sdl2::mixer::Music::resume();
    }

    /// Check if music is playing
    pub fn is_music_playing(&self) -> bool {
        sdl2::mixer::Music::is_playing()
    }

    /// Check if music is paused
    pub fn is_music_paused(&self) -> bool {
        sdl2::mixer::Music::is_paused()
    }

    /// Set sound effect volume (0-128)
    pub fn set_sfx_volume(&mut self, volume: u32) -> Result<(), AudioError> {
        if volume > 128 {
            return Err(AudioError::InvalidVolume(volume));
        }
        self.sfx_volume = volume as i32;
        sdl2::mixer::Channel::all().set_volume(self.sfx_volume);
        Ok(())
    }

    /// Get sound effect volume (0-128)
    pub fn sfx_volume(&self) -> u32 {
        self.sfx_volume as u32
    }

    /// Set music volume (0-128)
    pub fn set_music_volume(&mut self, volume: u32) -> Result<(), AudioError> {
        if volume > 128 {
            return Err(AudioError::InvalidVolume(volume));
        }
        self.music_volume = volume as i32;
        sdl2::mixer::Music::set_volume(self.music_volume);
        Ok(())
    }

    /// Get music volume (0-128)
    pub fn music_volume(&self) -> u32 {
        self.music_volume as u32
    }

    /// Set volume for a specific channel (0-128)
    pub fn set_channel_volume(&self, channel: i32, volume: u32) -> Result<(), AudioError> {
        if volume > 128 {
            return Err(AudioError::InvalidVolume(volume));
        }
        if channel >= 0 && channel < self.num_channels {
            sdl2::mixer::Channel(channel).set_volume(volume as i32);
            Ok(())
        } else {
            Err(AudioError::PlaybackFailed(format!(
                "Invalid channel: {}",
                channel
            )))
        }
    }

    /// Get volume for a specific channel (0-128)
    pub fn channel_volume(&self, channel: i32) -> Option<u32> {
        if channel >= 0 && channel < self.num_channels {
            Some(sdl2::mixer::Channel(channel).get_volume() as u32)
        } else {
            None
        }
    }

    /// Get number of available sound channels
    pub fn num_channels(&self) -> i32 {
        self.num_channels
    }

    /// Get number of loaded sounds
    pub fn num_sounds(&self) -> usize {
        self.sounds.len()
    }

    /// Check if a sound ID is loaded
    pub fn has_sound(&self, sound_id: SoundId) -> bool {
        self.sounds.contains_key(&sound_id)
    }

    /// Unload a sound effect
    pub fn unload_sound(&mut self, sound_id: SoundId) -> Result<(), AudioError> {
        if self.sounds.remove(&sound_id).is_some() {
            Ok(())
        } else {
            Err(AudioError::InvalidSoundId(sound_id))
        }
    }

    /// Unload all sound effects
    pub fn unload_all_sounds(&mut self) {
        self.sounds.clear();
    }
}

impl Drop for AudioManager {
    fn drop(&mut self) {
        // Clean up audio resources
        self.stop_all_sounds();
        self.stop_music();
        self.unload_all_sounds();
        sdl2::mixer::close_audio();
    }
}

/// Global audio manager instance (thread-safe)
pub struct GlobalAudioManager(Mutex<Option<AudioManager>>);

impl GlobalAudioManager {
    /// Create new global audio manager
    pub fn new() -> Self {
        GlobalAudioManager(Mutex::new(None))
    }

    /// Initialize the global audio manager
    pub fn initialize(&self, config: &AudioConfig) -> Result<(), AudioError> {
        let mut guard = self.0.lock().unwrap();
        if guard.is_some() {
            return Err(AudioError::InitializationFailed(
                "Audio already initialized".to_string(),
            ));
        }
        *guard = Some(AudioManager::with_config(config)?);
        Ok(())
    }

    /// Execute a function on the audio manager
    pub fn with_manager<F, R>(&self, f: F) -> Result<R, AudioError>
    where
        F: FnOnce(&AudioManager) -> R,
    {
        let guard = self.0.lock().unwrap();
        guard
            .as_ref()
            .map(f)
            .ok_or(AudioError::NotInitialized)
    }

    /// Execute a mutable function on the audio manager
    pub fn with_manager_mut<F, R>(&self, f: F) -> Result<R, AudioError>
    where
        F: FnOnce(&mut AudioManager) -> R,
    {
        let mut guard = self.0.lock().unwrap();
        guard
            .as_mut()
            .map(f)
            .ok_or(AudioError::NotInitialized)
    }

    /// Check if audio is initialized
    pub fn is_initialized(&self) -> bool {
        self.0.lock().unwrap().is_some()
    }

    /// Shutdown the audio manager
    pub fn shutdown(&self) {
        let mut guard = self.0.lock().unwrap();
        *guard = None;
    }
}

impl Default for GlobalAudioManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_config_default() {
        let config = AudioConfig::default();
        assert_eq!(config.frequency, DEFAULT_FREQUENCY);
        assert_eq!(config.channels, DEFAULT_CHANNELS);
        assert_eq!(config.chunk_size, 2048);
    }

    #[test]
    fn test_audio_config_builder() {
        let config = AudioConfig::new()
            .with_frequency(48000)
            .with_format(AudioFormat::F32)
            .with_channels(2)
            .with_chunk_size(4096);

        assert_eq!(config.frequency, 48000);
        assert_eq!(config.format, AudioFormat::F32);
        assert_eq!(config.channels, 2);
        assert_eq!(config.chunk_size, 4096);
    }

    #[test]
    fn test_audio_format_to_sdl2() {
        assert_eq!(AudioFormat::U8.to_sdl2_format(), AUDIO_U8);
        assert_eq!(AudioFormat::S8.to_sdl2_format(), AUDIO_S8);
        assert_eq!(AudioFormat::U16LSB.to_sdl2_format(), AUDIO_U16LSB);
        assert_eq!(AudioFormat::U16MSB.to_sdl2_format(), AUDIO_U16MSB);
        assert_eq!(AudioFormat::S16LSB.to_sdl2_format(), AUDIO_S16LSB);
        assert_eq!(AudioFormat::S16MSB.to_sdl2_format(), AUDIO_S16MSB);
        assert_eq!(AudioFormat::F32.to_sdl2_format(), AUDIO_F32);
    }

    #[test]
    fn test_audio_error_display() {
        let err = AudioError::InvalidVolume(150);
        assert!(err.to_string().contains("Invalid volume"));
    }

    #[test]
    fn test_audio_manager_creation() {
        // Note: This test requires SDL2_mixer to be available
        // It may fail in some environments
        let result = AudioManager::new();
        // We can't assert success here as it depends on system setup
        // But we can verify the function doesn't panic
        let _ = result;
    }

    #[test]
    fn test_global_audio_manager() {
        let global = GlobalAudioManager::new();
        assert!(!global.is_initialized());

        // Try to use uninitialized manager
        let result = global.with_manager(|_| ());
        assert!(matches!(result, Err(AudioError::NotInitialized)));

        // Shutdown uninitialized manager should not panic
        global.shutdown();
    }

    #[test]
    fn test_audio_format_equality() {
        assert_eq!(AudioFormat::S16LSB, AudioFormat::S16LSB);
        assert_ne!(AudioFormat::U8, AudioFormat::S8);
    }
}
