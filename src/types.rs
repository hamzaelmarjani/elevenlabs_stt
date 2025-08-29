use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct STTRequest {
    // The file to transcribe. All major audio and video formats are supported.
    // Exactly one of the `file` or `cloud_storage_url` parameters must be provided.
    // The file size must be less than 3.0GB.
    // If this is None, you must provide `cloud_storage_url`.
    pub file: Option<Vec<u8>>,

    // The ID of the model to use for transcription.
    // Currently only `scribe_v1` and `scribe_v1_experimental` are available.
    pub model_id: String,

    // Language code (ISO 639-1) used to enforce a language for the model. Currently only Turbo v2.5 and Flash v2.5 support language enforcement.
    // For other models, an error will be returned if language code is provided.
    // You can see all supported languages for each model: https://help.elevenlabs.io/hc/en-us/articles/13313366263441-What-languages-do-you-support
    // Note: this parameter in ElevenLabs API doesn't translate text - it only controls the pronunciation/accent when speaking the text.
    // The text itself remains in the original language. i.e: If you want French audio, you need to provide French text.
    pub language_code: Option<String>,

    // Whether to tag audio events like (laughter), (footsteps), etc. in the transcription.
    // Defaults to true
    pub tag_audio_events: Option<bool>,

    // The maximum amount of speakers talking in the uploaded file. Can help with predicting who speaks when.
    // The maximum amount of speakers that can be predicted is 32.
    // Defaults to None, in this case the amount of speakers is set to the maximum value the model supports.
    pub num_speakers: Option<u32>,

    // The granularity of the timestamps in the transcription. ‘word’ provides word-level timestamps and ‘character’ provides character-level timestamps per word.
    // Allowed values: none, word, character. Defaults to word.
    pub timestamps_granularity: Option<String>,

    // Whether to annotate which speaker is currently talking in the uploaded file.
    // Defaults to false
    pub diarize: Option<bool>,

    // Diarization threshold to apply during speaker diarization. A higher value means there will be a lower chance of one speaker being diarized as two different speakers
    // But also a higher chance of two different speakers being diarized as one speaker (less total speakers predicted). A low value means there will be a higher chance
    // Of one speaker being diarized as two different speakers but also a lower chance of two different speakers being diarized as one speaker (more total speakers predicted).
    // Can only be set when diarize=True and num_speakers=None.
    // Defaults to None, in which case we will choose a threshold based on the model_id (0.22 usually).
    pub diarization_threshold: Option<f32>,

    // The HTTPS URL of the file to transcribe. Exactly one of the file or cloud_storage_url parameters must be provided. The file must be accessible via HTTPS and the file size must be less than 2GB.
    // Any valid HTTPS URL is accepted, including URLs from cloud storage providers (AWS S3, Google Cloud Storage, Cloudflare R2, etc.), CDNs, or any other HTTPS source.
    // URLs can be pre-signed or include authentication tokens in query parameters.
    // If this is None, you must provide `file`.
    pub cloud_storage_url: Option<String>,

    // Whether to send the transcription result to configured speech-to-text webhooks.
    // If set the request will return early without the transcription, which will be delivered later via webhook.
    // Defaults to false
    pub webhook: Option<bool>,

    // Optional specific webhook ID to send the transcription result to.
    // Only valid when webhook is set to true. If not provided, transcription will be sent to all configured speech-to-text webhooks.
    pub webhook_id: Option<String>,

    // Controls the randomness of the transcription output.
    // Accepts values between 0.0 and 2.0, where higher values result in more diverse and less deterministic results.
    // If omitted, we will use a temperature based on the model you selected which is usually 0.
    pub temperature: Option<f32>,

    // If specified, our system will make a best effort to sample deterministically,
    // Such that repeated requests with the same seed and parameters should return the same result.
    // Determinism is not guaranteed. Must be an integer between 0 and 2147483647.
    // Defaults to None
    pub seed: Option<u32>,

    // Whether the audio file contains multiple channels where each channel contains a single speaker.
    // When enabled, each channel will be transcribed independently and the results will be combined.
    // Each word in the response will include a ‘channel_index’ field indicating which channel it was spoken on.
    // A maximum of 5 channels is supported.
    // Defaults to false
    pub use_multi_channel: Option<bool>,

    // Optional metadata to be included in the webhook response.
    // This should be a JSON string representing an object with a maximum depth of 2 levels and maximum size of 16KB.
    // Useful for tracking internal IDs, job references, or other contextual information.
    // Defaults to None
    pub webhook_metadata: Option<String>,
}

/// Voice settings for fine-tuning speech output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct STTResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_probability: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub words: Option<Vec<STTResponseWord>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct STTResponseWord {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprob: Option<f32>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_field: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speaker_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub characters: Option<Vec<STTResponseWordCharacters>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct STTResponseWordCharacters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<f32>,
}
