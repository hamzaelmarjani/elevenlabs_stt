use elevenlabs_stt::{ElevenLabsSTTClient, STTResponse, models};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get API key from environment variable
    let api_key =
        env::var("ELEVENLABS_API_KEY").expect("Please set ELEVENLABS_API_KEY environment variable");

    // Creating ElevenLabs client
    let client = ElevenLabsSTTClient::new(api_key);

    // Get audio file bytes
    let file_path = "inputs/speech.mp3";
    let file_content = std::fs::read(file_path)?;

    // Run speech to text execution
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

    // Handle response
    println!("Results: {:?}", stt_reponse);

    Ok(())
}
