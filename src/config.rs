use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct VoiceVoxConfig {
    pub url: String,
    pub max_retry: u64,
    pub default_speed: f64,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
pub struct TransformConfig {
    pub url_alternative_text: String,
    pub max_length: usize,
    pub ellipsis_text: String,
    pub read_name: bool,
    pub max_name_length: usize,
    pub ellipsis_name_text: String,
    pub read_NIP36: bool,
}

#[derive(Deserialize, Debug)]
pub struct NostrConfig {
    pub relays: Vec<String>,
    pub old_threshold_seconds: u64,
    pub pubkey: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub voicevox: VoiceVoxConfig,
    pub nostr: NostrConfig,
    pub speaker: u32,
    pub transform: TransformConfig,
}
