use elevenlabs_stt::{ElevenLabsSTTClient, STTResponse};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get API key from environment variable
    let api_key =
        env::var("ELEVENLABS_API_KEY").expect("Please set ELEVENLABS_API_KEY environment variable");

    // Creating ElevenLabs client
    let client = ElevenLabsSTTClient::new(api_key);

    // Test Basic STT with new voice API
    println!("Converting speech to text started ...");

    // Get audio file bytes
    let file_path = "inputs/speech.mp3";
    let file_content = std::fs::read(file_path)?;

    // Run speech to text execution
    let stt_reponse: STTResponse = client.speech_to_text(file_content).execute().await?;

    // Handle response
    println!("Results: {:?}", stt_reponse);

    Ok(())
}
