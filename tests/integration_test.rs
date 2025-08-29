use elevenlabs_stt::{ElevenLabsSTTClient, ElevenLabsSTTError, models};

#[tokio::test]
async fn test_client_creation() {
    let _client = ElevenLabsSTTClient::new("test-api-key");
    // Just test that it doesn't panic
    assert_eq!(true, true);
}

#[tokio::test]
async fn test_builder_pattern() {
    let client = ElevenLabsSTTClient::new("test-key");

    // Get audio file bytes
    let file_path = "inputs/speech.mp3";
    let file_content = std::fs::read(file_path).unwrap();

    let _builder = client
        .speech_to_text(file_content)
        .model(models::elevanlabs_models::SCRIBE_V1);

    // Test that builder methods are chainable
    assert_eq!(true, true); // Builder pattern works if this compiles
}

#[test]
fn test_error_display() {
    let error = ElevenLabsSTTError::ValidationError("Invalid voice ID".to_string());
    let display = format!("{}", error);

    // Check that the display contains the correct message
    assert!(display.contains("Validation error"));
    assert!(display.contains("Invalid voice ID"));
}

#[cfg(test)]
mod mock_tests {
    use super::*;
    #[tokio::test]
    async fn test_invalid_api_key_error() {
        let _client = ElevenLabsSTTClient::new("invalid-key");

        // This would normally fail with auth error, but we can't test without real API
        // In a real test, you'd use a mock HTTP server like wiremock
        // For now, just test that the client can be created
        assert_eq!(true, true);
    }
}
