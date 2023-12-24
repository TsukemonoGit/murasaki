use anyhow::{anyhow, Context};
use log::{info, warn};
use rodio::Decoder;
use std::io::{BufReader, Cursor};

use crate::config::VoiceVoxConfig;
use crate::voicevox::{self, AudioQuery};

pub struct TTS {
    vv: voicevox::Client,
    max_retry: u64,
    sink: rodio::Sink,
}

impl TTS {
    pub fn new(sink: rodio::Sink, voicevox_config: &VoiceVoxConfig) -> Self {
        let vv = voicevox::Client::new(&voicevox_config.url);
        let max_retry = voicevox_config.max_retry;
        Self {
            vv,
            max_retry,
            sink,
        }
    }
    #[allow(non_snake_case)]
    pub async fn say(&self, speaker: u32, text: &String, speed: f64) -> anyhow::Result<()> {
        let speedScale = speed + (self.sink.len() as f64 / 10.0);
        let speed_scale_formatted = format!("{:.1}", speedScale);
        info!("📣 ({}) {}", speed_scale_formatted, text);
        //info!("📣 ({}) {}", speedScale, text);
        //println!("len: {}", len);

        let query = self
            .vv
            .audio_query(speaker, &text)
            .await
            .context("failed in audio_query")?;
        // JSON文字列をAudioQueryオブジェクトに変換
        let mut audio_query: AudioQuery = serde_json::from_str(&query)?;
        audio_query.speedScale = speedScale;
        // 修正したValueをJSON文字列に変換
        let modified_json: String = serde_json::to_string(&audio_query)?;

        for _retry in 0..self.max_retry {
            match self.vv.synthesis(speaker, &modified_json).await {
                Err(e) => {
                    warn!("error in synthesis: {}", e);
                }
                Ok(wav) => {
                    let content = Cursor::new(wav);
                    let file = BufReader::new(content);
                    let source = Decoder::new_wav(file);
                    match source {
                        Ok(source) => {
                            // self.sink.set_speed(speed); //これボイスボックスのスペードかえるやつじゃなくてオーディオのほうのスピードチェンジのよかん
                            self.sink.append(source);

                            return Ok(());
                        }
                        Err(e) => {
                            warn!("failed to decode wav: {}", e);
                        }
                    }
                }
            }
        }

        Err(anyhow!("synthesis retry limit exceeded"))
    }
}
