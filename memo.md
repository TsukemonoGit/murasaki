## 追加した事項

- pubkey ごとのスピーカー設定

- デフォルト話速の設定を追加

- 未読み上げのノートがたまるほど話速が上がる

- name の読み上げ文字数を追加

- NIP-36 の読み上げ有無

## pubkey ごとにスピーカーを設定

speaker 番号は[対応表](https://github.com/VOICEVOX/voicevox_fat_resource/blob/main/core/model/README.md#%E9%9F%B3%E5%A3%B0%E3%83%A2%E3%83%87%E3%83%ABvvm%E3%83%95%E3%82%A1%E3%82%A4%E3%83%AB%E3%81%A8%E5%A3%B0%E3%82%AD%E3%83%A3%E3%83%A9%E3%82%AF%E3%82%BF%E3%83%BC%E3%82%B9%E3%82%BF%E3%82%A4%E3%83%AB%E5%90%8D%E3%81%A8%E3%82%B9%E3%82%BF%E3%82%A4%E3%83%AB-id-%E3%81%AE%E5%AF%BE%E5%BF%9C%E8%A1%A8)のスタイル ID 番号

speakers.json 設定していない番号は config.toml の speaker が喋る

## 話速設定までの学びをメモ

VOICEVOX Engine の API
http://localhost:50021/docs

#### 音声合成の流れ

1. クエリ作成

   voicevox.rs の`/audio_query`のところで、speaker 番号と text を渡すと、
   こんな感じの AudioQuery で application/json 形式な String が帰ってくる

   ```
   {
     "accent_phrases": [
       {
         "moras": [
           {
             "text": "string",
             "consonant": "string",
             "consonant_length": 0,
             "vowel": "string",
             "vowel_length": 0,
             "pitch": 0
           }
         ],
         "accent": 0,
         "pause_mora": {
           "text": "string",
           "consonant": "string",
           "consonant_length": 0,
           "vowel": "string",
           "vowel_length": 0,
           "pitch": 0
         },
         "is_interrogative": false
       }
     ],
     "speedScale": 0,
     "pitchScale": 0,
     "intonationScale": 0,
     "volumeScale": 0,
     "prePhonemeLength": 0,
     "postPhonemeLength": 0,
     "outputSamplingRate": 0,
     "outputStereo": true,
     "kana": "string"
   }
   ```

2. この JSON のいろんな値をいじったりいじらなかったりする
   (ここで speedScale を調整)

3. String になおして`/synthesis`になげる

4. （成功したら）音声合成後の`audio/wav`が帰ってくる
