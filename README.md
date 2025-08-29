# elevenlabs_stt

[![Crates.io](https://img.shields.io/crates/v/elevenlabs_stt.svg)](https://crates.io/crates/elevenlabs_stt)
[![Docs.rs](https://docs.rs/elevenlabs_stt/badge.svg)](https://docs.rs/elevenlabs_stt)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue)](#license)

A type-safe, async Rust client for the [ElevenLabs Speech-to-Text API](https://elevenlabs.io/app/speech-to-text). Transcribe audio and videos to text with a simple, ergonomic API.

## Features

- **Type-safe & Async**: Built with Rust's type system and async/await support
- **Builder Pattern**: Intuitive, chainable API for configuring STT requests
- **Model Support**: Full support for ElevenLabs models (`models::elevenlabs_models::*`)
- **Customizable**: Elevanlabs STT APIs, custom base URLs, and enterprise support
- **Tokio Ready**: Works seamlessly with the Tokio runtime
- **Audio & Video**: Works with audios and videos, up to 3.0GB

## Check-out Also:

**This project is part of a milestone to implement all ElevenLabs APIs in Rust.**

- **[Elevenlabs TTS](https://crates.io/crates/elevenlabs_tts)**: ElevenLabs Text-to-Speech API. ✅
- **[Elevenlabs TTD](https://crates.io/crates/elevenlabs_ttd)**: ElevenLabs Text-to-Dialogue API. ✅
- **[Elevenlabs STT](https://crates.io/crates/elevenlabs_stt)**: ElevenLabs Speech-to-Text API. ✅
- **[Elevenlabs SFX](https://crates.io/crates/elevenlabs_sfx)**: ElevenLabs Sound Effects API. ✅
- **Elevenlabs TTV**: ElevenLabs Text-to-Voice API. ⏳
- **Elevenlabs VC**: ElevenLabs Voice Changer API. ⏳
- **Elevenlabs CM**: ElevenLabs Music Compose API. ⏳
- **Elevenlabs AUI**: ElevenLabs Audio Isolation API. ⏳
- **Elevenlabs DUB**: ElevenLabs Dubbing API. ⏳

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
elevenlabs_stt = "0.0.2"
```

## Quick Start

```rust
use elevenlabs_stt::{ElevenLabsSTTClient, STTResponse};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ElevenLabsSTTClient::new("your-api-key");

    let file_path = "inputs/speech.mp3";
    let file_content = std::fs::read(file_path)?;

    let stt_reponse: STTResponse = client.speech_to_text(file_content).execute().await?;

    println!("Results: {:?}", stt_reponse);
    Ok(())
}
```

## Examples

### Basic Usage

```rust
use elevenlabs_stt::{ElevenLabsSTTClient, STTResponse, models, voices};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key =
        env::var("ELEVENLABS_API_KEY").expect("Please set ELEVENLABS_API_KEY environment variable");

    let client = ElevenLabsSTTClient::new(api_key);

    let file_path = "inputs/speech.mp3";
    let file_content = std::fs::read(file_path)?;

    let stt_reponse: STTResponse = client.speech_to_text(file_content).execute().await?;
    println!("Results: {:?}", stt_reponse);
    Ok(())
}
```

### Advanced Configuration

```rust
use elevenlabs_stt::{ElevenLabsSTTClient, VoiceSettings, STTResponse, models, voices};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key =
        env::var("ELEVENLABS_API_KEY").expect("Please set ELEVENLABS_API_KEY environment variable");

    let client = ElevenLabsSTTClient::new(api_key);

    let file_path = "inputs/speech.mp3";
    let file_content = std::fs::read(file_path)?;

    let stt_reponse: STTResponse = client
        .speech_to_text(file_content)
        .model(models::elevanlabs_models::SCRIBE_V1)
        .language_code("en")
        .tag_audio_events(true)
        .timestamps_granularity("word")
        .diarize(true)
        .diarization_threshold(0.22)
        .webhook(false)
        .webhook(false)
        .temperature(0.2)
        .seed(4000)
        .use_multi_channel(false)
        .execute()
        .await?;

    println!("Results: {:?}", stt_reponse);

    Ok(())
}
```

### Running Examples

```bash
# Set your API key
export ELEVENLABS_API_KEY=your_api_key_here

# Run the basic example
cargo run --example basic_stt

# Run the advanced example
cargo run --example advanced_stt
```

## API Overview

| Method                             | Description                                                                         |
| ---------------------------------- | ----------------------------------------------------------------------------------- |
| `ElevenLabsSTTClient::new(String)` | Create client instance (required)\*                                                 |
| `.speech_to_text(Option<Vec<u8>>)` | Build a STT request, (File or `cloud_storage_url`) (required)\*                     |
| `.model(String)`                   | Select model (optional)                                                             |
| `.language_code(String)`           | Force language pronounce/accent only (no translation) (optional)                    |
| `.tag_audio_events(bool)`          | Tag audio events like (laughter), (footsteps), etc. (optional)                      |
| `.num_speakers(u32)`               | The max amount of speakers talking in the uploaded file. (optional)                 |
| `.timestamps_granularity(String)`  | Allowed values: none, word, character. Defaults to word. (optional)                 |
| `.diarize(bool)`                   | Which speaker is currently talking in the uploaded file. (optional)                 |
| `.diarization_threshold(f32)`      | Can only be set when diarize=True and num_speakers=None. (optional)                 |
| `.cloud_storage_url(String)`       | URL of the file to transcribe, if this is None, you must provide `file`. (optional) |
| `.webhook(bool)`                   | Send the transcription result to configured speech-to-text webhooks. (optional)     |
| `.webhook_id(String)`              | Optional specific webhook ID to send the transcription result to. (optional)        |
| `.temperature(f32)`                | Controls the randomness of the transcription output, between 0.0 and 2.0 (optional) |
| `.seed(u32)`                       | Our system will make a best effort to sample deterministically (optional)           |
| `.use_multi_channel(bool)`         | Whether the audio file contains multiple channels (optional)                        |
| `.webhook_metadata(String)`        | Optional metadata to be included in the webhook response (optional)                 |
| `.execute()`                       | Run request → transcribe file (required)\*                                          |

## Error Handling

The crate uses standard Rust error handling patterns. All async methods return `Result` types:

```rust
match client.speech_to_text(file).execute().await {
    Ok(audio) => println!("Transcribed text from file: {}", result.text),
    Err(e) => eprintln!("STT transcription failed: {}", e),
}
```

## Requirements

- Rust 1.70+ (for async/await support)
- Tokio runtime
- Valid ElevenLabs API key

## License

Licensed under either of:

- [MIT License](LICENSE-MIT)
- [Apache License, Version 2.0](LICENSE-APACHE)

at your option.

## Contributing

Contributions are welcome! Please feel free to:

- Open issues for bugs or feature requests
- Submit pull requests with improvements
- Improve documentation or examples
- Add tests or benchmarks

Before contributing, please ensure your code follows Rust conventions and includes appropriate tests.

## Support

If you like this project, consider supporting me on Patreon 💖

[![Patreon](https://img.shields.io/badge/Support-Patreon-orange.svg)](https://www.patreon.com/elmarjanihamza/gift)

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for a detailed history of changes.

---

**Note**: This crate is not officially affiliated with ElevenLabs. Please refer to the [ElevenLabs API documentation](https://elevenlabs.io/docs/api-reference/speech-to-text/convert) for the most up-to-date API information.
