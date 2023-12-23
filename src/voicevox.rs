use bytes::Bytes;
use serde::{Deserialize, Serialize};

pub struct Client {
    client: reqwest::Client,
    voicevox_url: String,
}

impl Client {
    pub fn new(voicevox_url: &String) -> Self {
        Client {
            client: reqwest::Client::new(),
            voicevox_url: voicevox_url.clone(),
        }
    }

    pub async fn audio_query(&self, speaker: u32, text: &String) -> Result<String, reqwest::Error> {
        let resp = self
            .client
            .post(self.voicevox_url.clone() + "/audio_query")
            .query(&[("speaker", &speaker.to_string()), ("text", &text)])
            .send()
            .await?
            .text()
            .await?;

        Ok(resp)
    }

    pub async fn synthesis(&self, speaker: u32, query: &String) -> Result<Bytes, reqwest::Error> {
        let resp = self
            .client
            .post(self.voicevox_url.clone() + "/synthesis")
            .query(&[("speaker", &speaker.to_string())])
            .body(query.clone())
            .send()
            .await?
            .bytes()
            .await?;

        Ok(resp)
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Mora {
    pub text: String,
    pub vowel: String,
    pub vowel_length: f64,
    pub pitch: f64,
    pub consonant: Option<String>,
    pub consonant_length: Option<f64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AccentPhrase {
    pub moras: Vec<Mora>,
    pub accent: i32,
    pub pause_mora: Option<Mora>,
    pub is_interrogative: bool,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize)]
pub struct AudioQuery {
    pub accent_phrases: Vec<AccentPhrase>,
    pub speedScale: f64,
    pub pitchScale: f64,
    pub intonationScale: f64,
    pub volumeScale: f64,
    pub prePhonemeLength: f64,
    pub postPhonemeLength: f64,
    pub outputSamplingRate: i32,
    pub outputStereo: bool,
    pub kana: Option<String>,
}

impl Default for AudioQuery {
    fn default() -> Self {
        AudioQuery {
            accent_phrases: vec![AccentPhrase {
                moras: vec![Mora {
                    text: String::default(),
                    vowel: String::default(),
                    vowel_length: 0.0,
                    pitch: 0.0,
                    consonant: None,
                    consonant_length: None,
                }],
                accent: 0,
                pause_mora: Some(Mora {
                    text: String::default(),
                    vowel: String::default(),
                    vowel_length: 0.0,
                    pitch: 0.0,
                    consonant: None,
                    consonant_length: None,
                }),
                is_interrogative: false,
            }],
            speedScale: 0.0,
            pitchScale: 0.0,
            intonationScale: 0.0,
            volumeScale: 0.0,
            prePhonemeLength: 0.0,
            postPhonemeLength: 0.0,
            outputSamplingRate: 0,
            outputStereo: true,
            kana: None,
        }
    }
}
