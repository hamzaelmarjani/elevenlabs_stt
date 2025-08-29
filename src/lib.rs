//! ElevenLabs Speech-To-Text API client
//!
//! A type-safe, async Rust client for the ElevenLabs STT API.
//!
//! # Quick Start
//!
//! ```rust,no_run
//! use elevenlabs_stt::ElevenLabsSTTClient;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = ElevenLabsSTTClient::new("your-api-key");
//!
//!     let file_path = "inputs/speech.mp3";
//!     let file_content = std::fs::read(file_path)?;
//!     
//!     let stt_reponse = client.speech_to_text(file_content).execute().await?;
//!     
//!     println!("Transcription: {:?}", stt_reponse.text);
//!     Ok(())
//! }
//! ```

use reqwest::Client;

pub mod error;
pub mod models;
pub mod types;

pub use error::ElevenLabsSTTError;
pub use types::*;

/// Main client for interacting with ElevenLabs API
#[derive(Clone)]
pub struct ElevenLabsSTTClient {
    client: Client,
    api_key: String,
    base_url: String,
}

impl ElevenLabsSTTClient {
    /// Create a new ElevenLabs client with API key
    pub fn new<S: Into<String>>(api_key: S) -> Self {
        Self {
            client: Client::new(),
            api_key: api_key.into(),
            base_url: "https://api.elevenlabs.io/v1".to_string(),
        }
    }

    /// Create a new client with custom base URL (for testing/enterprise)
    pub fn with_base_url<S: Into<String>>(api_key: S, base_url: S) -> Self {
        Self {
            client: Client::new(),
            api_key: api_key.into(),
            base_url: base_url.into(),
        }
    }

    /// Start building a speech-to-text request
    pub fn speech_to_text<F: Into<Option<Vec<u8>>>>(&self, file: F) -> SpeechToTextBuilder {
        SpeechToTextBuilder::new(self.clone(), file.into())
    }

    /// Internal method to execute STT request
    pub(crate) async fn execute_stt(
        &self,
        request: STTRequest,
    ) -> Result<STTResponse, ElevenLabsSTTError> {
        let mut form = reqwest::multipart::Form::new().text("model_id", request.model_id);

        if let Some(file_data) = request.file {
            let part = reqwest::multipart::Part::bytes(file_data)
                .file_name("file")
                .mime_str("application/octet-stream")
                .map_err(|e| ElevenLabsSTTError::RequestError(e));

            match part {
                Ok(part) => form = form.part("file", part),
                Err(e) => return Err(e),
            }
        }

        let request_fields = vec![
            ("language_code", request.language_code.map(|n| n)),
            (
                "tag_audio_events",
                request.tag_audio_events.map(|n| n.to_string()),
            ),
            ("num_speakers", request.num_speakers.map(|n| n.to_string())),
            (
                "timestamps_granularity",
                request.timestamps_granularity.map(|n| n),
            ),
            ("diarize", request.diarize.map(|n| n.to_string())),
            (
                "diarization_threshold",
                request.diarization_threshold.map(|n| n.to_string()),
            ),
            ("cloud_storage_url", request.cloud_storage_url.map(|n| n)),
            ("webhook", request.webhook.map(|n| n.to_string())),
            ("webhook_id", request.webhook_id.map(|n| n)),
            ("temperature", request.temperature.map(|n| n.to_string())),
            ("seed", request.seed.map(|n| n.to_string())),
            (
                "use_multi_channel",
                request.use_multi_channel.map(|n| n.to_string()),
            ),
            (
                "webhook_metadata",
                request.webhook_metadata.map(|n| n.to_string()),
            ),
        ];

        for (key, value) in request_fields {
            if let Some(val) = value {
                form = form.text(key, val);
            }
        }

        let url = format!("{}/speech-to-text", self.base_url);

        let response = self
            .client
            .post(&url)
            .header("xi-api-key", &self.api_key)
            .multipart(form)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(ElevenLabsSTTError::ApiError {
                status: response.status().as_u16(),
                message: response.text().await.unwrap_or_default(),
            });
        }

        let parse_response = response.json::<STTResponse>().await;

        match parse_response {
            Ok(stt_response) => return Ok(stt_response),
            Err(e) => return Err(ElevenLabsSTTError::ParseError(e)),
        }
    }
}

pub struct SpeechToTextBuilder {
    client: ElevenLabsSTTClient,
    file: Option<Vec<u8>>,
    model_id: Option<String>,
    language_code: Option<String>,
    tag_audio_events: Option<bool>,
    num_speakers: Option<u32>,
    timestamps_granularity: Option<String>,
    diarize: Option<bool>,
    diarization_threshold: Option<f32>,
    cloud_storage_url: Option<String>,
    webhook: Option<bool>,
    webhook_id: Option<String>,
    temperature: Option<f32>,
    seed: Option<u32>,
    webhook_metadata: Option<String>,
    use_multi_channel: Option<bool>,
}

impl SpeechToTextBuilder {
    fn new(client: ElevenLabsSTTClient, file: Option<Vec<u8>>) -> Self {
        Self {
            client,
            file,
            model_id: None,
            language_code: None,
            tag_audio_events: None,
            num_speakers: None,
            timestamps_granularity: None,
            diarize: None,
            diarization_threshold: None,
            cloud_storage_url: None,
            webhook: None,
            webhook_id: None,
            temperature: None,
            seed: None,
            use_multi_channel: None,
            webhook_metadata: None,
        }
    }

    /// Set the model to use
    pub fn model<S: Into<String>>(mut self, model_id: S) -> Self {
        self.model_id = Some(model_id.into());
        self
    }

    /// Set the language code to use
    pub fn language_code<S: Into<String>>(mut self, language_code: S) -> Self {
        self.language_code = Some(language_code.into());
        self
    }

    /// Set the tag audio events to use
    pub fn tag_audio_events<B: Into<bool>>(mut self, tag_audio_events: B) -> Self {
        self.tag_audio_events = Some(tag_audio_events.into());
        self
    }

    /// Set the num speakers to use
    pub fn num_speakers(mut self, num_speakers: u32) -> Self {
        self.num_speakers = Some(num_speakers);
        self
    }

    /// Set the timestamps granularity to use
    pub fn timestamps_granularity<S: Into<String>>(mut self, timestamps_granularity: S) -> Self {
        self.timestamps_granularity = Some(timestamps_granularity.into());
        self
    }

    /// Set the diarize to use
    pub fn diarize<B: Into<bool>>(mut self, diarize: B) -> Self {
        self.diarize = Some(diarize.into());
        self
    }

    /// Set the diarization threshold to use
    pub fn diarization_threshold<F: Into<f32>>(mut self, diarization_threshold: F) -> Self {
        self.diarization_threshold = Some(diarization_threshold.into());
        self
    }

    /// Set the cloud storage url to use
    pub fn cloud_storage_url<S: Into<String>>(mut self, cloud_storage_url: S) -> Self {
        self.cloud_storage_url = Some(cloud_storage_url.into());
        self
    }

    /// Set the webhook to use
    pub fn webhook<B: Into<bool>>(mut self, webhook: B) -> Self {
        self.webhook = Some(webhook.into());
        self
    }

    /// Set the webhook id to use
    pub fn webhook_id<S: Into<String>>(mut self, webhook_id: S) -> Self {
        self.webhook_id = Some(webhook_id.into());
        self
    }

    /// Set the temperature to use
    pub fn temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature);
        self
    }

    /// Set the seed to use
    pub fn seed(mut self, seed: u32) -> Self {
        self.seed = Some(seed);
        self
    }

    /// Set the use multi channel to use
    pub fn use_multi_channel<B: Into<bool>>(mut self, use_multi_channel: B) -> Self {
        self.use_multi_channel = Some(use_multi_channel.into());
        self
    }

    /// Set the webhook metadata to use
    pub fn webhook_metadata<S: Into<String>>(mut self, webhook_metadata: S) -> Self {
        self.webhook_metadata = Some(webhook_metadata.into());
        self
    }

    /// Execute the speech-to-text request
    pub async fn execute(self) -> Result<STTResponse, ElevenLabsSTTError> {
        let request = STTRequest {
            file: self.file,
            model_id: self
                .model_id
                .unwrap_or_else(|| models::elevanlabs_models::SCRIBE_V1.to_string()),
            language_code: self.language_code,
            tag_audio_events: self.tag_audio_events,
            num_speakers: self.num_speakers,
            timestamps_granularity: self.timestamps_granularity,
            diarize: self.diarize,
            diarization_threshold: self.diarization_threshold,
            cloud_storage_url: self.cloud_storage_url,
            webhook: self.webhook,
            webhook_id: self.webhook_id,
            temperature: self.temperature,
            seed: self.seed,
            use_multi_channel: self.use_multi_channel,
            webhook_metadata: self.webhook_metadata,
        };

        self.client.execute_stt(request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_client_creation() {
        let client = ElevenLabsSTTClient::new("test-key");
        assert_eq!(client.api_key, "test-key");
    }

    #[test]
    fn test_builder_pattern() {
        let client = ElevenLabsSTTClient::new("test-key");
        let builder = client
            .speech_to_text(None)
            .model(models::elevanlabs_models::SCRIBE_V1);

        // Builder pattern works
        assert_eq!(builder.file, None);
        assert_eq!(
            builder.model_id,
            Some(models::elevanlabs_models::SCRIBE_V1.to_string())
        );
    }
}
