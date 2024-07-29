
use tetra::audio::{Sound, SoundInstance, SoundState};
use std::fs;
use std::fs::File;
use std::io::{BufReader, Read};
pub struct AudioManager
{
    sounds: HashMap<String, Arc<SoundInstance>>,
    channels: Vec<String>,
}

impl AudioManager {
    pub fn new() -> Self {
        let mut channels = Vec::new();
        for _ in 0..48 {
            channels.push(String::new());
        }
        AudioManager {
            sounds: HashMap::new(),
            channels,
        }
    }

    pub fn load_audio(&mut self, ctx: &mut Context, id: &str, file_path: &str) {
        if (!file_path.contains(".wav")) {
            let file = File::open(file_path).unwrap();
            let mut reader = BufReader::new(file);
            let mut buffer = Vec::new();
            reader.read_to_end(&mut buffer).unwrap();
            let sound = Sound::from_encoded(&buffer).spawn(ctx).expect(&*("GAY SOUND!!!!".to_owned() + file_path));
            self.sounds.insert(id.to_string(), Arc::new(sound));
        }
    }

    pub fn load_audio_assets(&mut self, ctx: &mut Context, assets: &PathBuf) {
        match fs::read_dir(assets.join("sounds")) {
            Ok(entries) => {
                for entry in entries {
                    match entry {
                        Ok(dir_entry) => {
                            let path = dir_entry.path();
                            if path.is_file() {
                                if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
                                    self.load_audio(ctx, &file_name.to_string(), path.to_str().unwrap());
                                }
                            }
                        },
                        Err(e) => println!("Error reading directory entry: {:?}", e),
                    }
                }
            },
            Err(e) => println!("{:?}", e),
        }
    }

    pub fn play(&mut self, id: &str, loop_audio: bool) {
        if let Some(sound) = self.sounds.get(id) {
            let sound = sound.clone();
            if let Some(channel) = self.get_available_channel() {
                self.channels[channel] = id.to_string();
                sound.play();
                if loop_audio {
                    sound.set_repeating(true);
                }
            }
        }
    }

    pub fn play_on_channel(&mut self, id: &str, loop_audio: bool, channel_idx: usize) {
        if let Some(sound) = self.sounds.get(id) {
            let sound = sound.clone();
            if channel_idx < self.channels.len() {
                let existing_soundname = &self.channels[channel_idx];
                if self.sounds.contains_key(existing_soundname) {
                    self.sounds[existing_soundname].clone().stop();
                }
                self.channels[channel_idx] = id.to_string();
                sound.play();
                if loop_audio {
                    sound.set_repeating(true);
                }
            } else {
                println!("Nonexistent sound: {}", channel_idx);
            }
        }
    }

    pub fn stop(&self, id: &str) {
        let mut sound = self.sounds[id].clone();
     //   self.channels[id] = String::new(); // figure this shit out
        if sound.state() == SoundState::Playing {
            sound.stop();
        }
    }

    pub fn stop_channel(&mut self, index: usize) {
        if index < self.channels.len() {
            let mut soundname = &self.channels[index];
            if self.sounds.contains_key(soundname) {
                let mut sound = self.sounds[soundname].clone();
                self.channels[index] = String::new();
                if sound.state() == SoundState::Playing {
                    sound.stop();
                }
            }
        } else {
            println!("Invalid player index: {}", index);
        }
    }

    pub fn kill_all(&mut self) {
        let mut update_idxs: Vec<usize> = Vec::new();
        for (index, soundname) in self.channels.iter().enumerate() {
            if self.sounds.contains_key(soundname) {
                let mut sound = self.sounds[soundname].clone();
                update_idxs.push(index);
                if sound.state() == SoundState::Playing {
                    sound.stop();
                }
            }
        }
        for idx in update_idxs
        {
            self.channels[idx] = String::new();
        }
    }

    pub fn set_channel_volume(&self, channel_idx: usize, volume: f32) {
        if channel_idx < self.channels.len() {
            let soundname = &self.channels[channel_idx];
            if self.sounds.contains_key(soundname) {
                self.sounds[soundname].clone().set_volume(volume);
            }
        } else {
            println!("Invalid player index: {}", channel_idx);
        }
    }

    pub fn set_all_volumes(&self, volume: f32) {
        for soundname in &self.channels {
            if self.sounds.contains_key(soundname) {
                let mut sound = self.sounds[soundname].clone();
                if sound.state() == SoundState::Playing {
                    sound.set_volume(volume);
                }
            }
        }
    }

    fn get_available_channel(&self) -> Option<usize> {
        for (index, soundname) in self.channels.iter().enumerate() {
            if !self.sounds.contains_key(soundname) {
                return Some(index);
            }
        }
        None
    }
}
