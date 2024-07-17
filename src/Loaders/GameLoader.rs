use std::fs::FileType;
use regex::Replacer;

pub mod GameLoader {
    use std::collections::HashMap;
    use std::io::Read;

    use serde::{Deserialize, Serialize};
    use serde::de::Error;
    use serde::Deserializer;
    use serde_json::Value;
    use std::fs;
    use std::path::PathBuf;
    use std::str::FromStr;

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Game {
        pub(crate) animatronics: HashMap<String, Animatronic>,
        pub(crate) cameras: HashMap<String, CamUI>,
        pub(crate) game_info: GameInfo,
        pub(crate) menus: HashMap<String, Menu>,
        pub(crate) offices: HashMap<String, Office>,
        pub(crate) sounds: Sounds,
        pub(crate) loaded_extensions: Vec<String>,
        #[serde(skip_deserializing)]
        pub(crate) office_scripts: HashMap<String, Vec<Code>>,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Animatronic
    {
        pub AI: Option<Vec<i32>>,
        #[serde(default)]
        pub Ignoremask: bool,
        pub Jumpscare: Option<Vec<String>>,
        pub path: Option<Vec<PathNode>>,
        #[serde(default)]
        pub State: String,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct PathNode {
        #[serde(default)]
        pub id: String,
        pub r#type: String,
        #[serde(default)]
        pub chance: i32,
        #[serde(default)]
        pub state: String,
        pub path: Option<Vec<PathNode>>,
        #[serde(default)]
        pub camid: String,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct CamUI {
        //buttons: HashMap<String, CamSprite>,
        // music_box: Vec<i32>,
        // sprites: HashMap<String, CamSprite>,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct CamSprite; // TODO: implement my campsite

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct GameInfo {
        id: String,
        title: String,
        style: i32,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Menu {
        pub(crate) code: Vec<Code>,
        #[serde(deserialize_with = "ignore_if_map")]
        pub(crate) elements: Vec<Element>,
        pub(crate) properties: Properties,
    }

    fn ignore_if_map<'de, D>(deserializer: D) -> Result<Vec<Element>, D::Error>
        where
            D: serde::Deserializer<'de>,
    {
        let value: Value = Deserialize::deserialize(deserializer)?;

        match &value {
            Value::Array(array) => {
                let elements: Result<Vec<Element>, _> = serde_json::from_value(value.clone());
                elements.map_err(|e| D::Error::custom(format!("Failed to deserialize elements: {}", e)))
            }
            Value::Object(_) => Ok(Vec::new()), // Skip deserializing if it's a map
            _ => Err(D::Error::custom("Unexpected JSON type for Vec<Element>")),
        }
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Code {
        #[serde(default)]
        pub negated: bool,
        pub args: Vec<Value>,
        pub block: String,
        #[serde(default)]
        pub subcode: Vec<Code>,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Element {
        #[serde(default)]
        pub Blue: i32,
        #[serde(default)]
        pub Red: i32,
        #[serde(default)]
        pub Green: i32,
        #[serde(default)]
        pub fontname: String,
        #[serde(default)]
        pub fontsize: i32,
        #[serde(default)]
        pub hidden: bool,
        #[serde(default)]
        pub id: String,
        #[serde(default)]
        pub text: String,
        pub r#type: String,
        #[serde(default)]
        pub(crate) sprite: String,
        #[serde(default)]
        pub animatronic: String,
        pub x: i32,
        pub y: i32,
        #[serde(default)]
        pub animation: String,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Properties {
        #[serde(default)]
        pub(crate) BackgroundImage: String,
        pub BackgroundColor: String,
        #[serde(default)]
        BackgroundMusic: String,
        pub ButtonArrows: bool,
        #[serde(default)]
        pub FadeIn: bool,
        #[serde(default)]
        pub MenuScroll: bool,
        #[serde(default)]
        pub FadeOut: bool,
        #[serde(default)]
        pub FadeSpeed: i32,
        // pub StaticEffect: Option<bool>,
        pub Panorama: bool,
        #[serde(default)]
        ButtonArrowStr: String,
        ButtonArrowColor: String,
        ButtonArrowFont: String,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Office {
        pub animations: Animations,
        pub flashlight: bool,
        pub mask: bool,
        pub(crate) objects: Vec<OfficeObject>,
        pub panorama: bool,
        pub power: Power,
        pub(crate) states: HashMap<String, String>,
        pub toxic: bool,
        pub uibuttons: Uibuttons,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Animations {
        camera: String,
        mask: String,
        powerout: String,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct OfficeObject {
        #[serde(default)]
        pub clickstyle: bool,
        pub id: String,
        #[serde(default)]
        pub on_sprite: String,
        pub position: Vec<i32>,
        #[serde(default)]
        pub close_sound: String,
        #[serde(default)]
        pub open_sound: String,
        #[serde(default)]
        pub animation: String,
        #[serde(default)]
        pub sound: String,
        #[serde(default)]
        pub sprite: String,
        #[serde(default)]
        pub trigger: Vec<i32>,
        #[serde(default)]
        pub r#type: String,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Power {
        animatronic: String,
        enabled: bool,
        starting_level: i32,
        ucn: bool,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Uibuttons {
        camera: CameraPanel,
        mask: MaskPanel,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct CameraPanel {
        image: String,
        position: Vec<i32>,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct MaskPanel {
        image: String,
        position: Vec<i32>,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Sounds {
        ambience: String,
        animatronic_move: Vec<String>,
        blip: String,
        camdown: String,
        camup: String,
        flashlight: String,
        maskbreathing: String,
        maskoff: String,
        maskon: String,
        masktoxic: String,
        music_box_run_out: String,
        phone_calls: Vec<String>,
        powerout: String,
        signal_interrupted: String,
        stare: String,
    }

    pub fn Load(input_json_path: &str) -> Game {
        let mut file = std::fs::File::open(input_json_path).expect("Failed to open the JSON file");
        let mut content = String::new();
        file.read_to_string(&mut content)
            .expect("Failed to read the JSON file");

        let mut settings = serde_json::from_str::<Game>(&content)
            .expect("Failed to deserialize JSON content");

        let mut scripts: HashMap<String, Vec<Code>> = HashMap::new();

        for script in fs::read_dir(input_json_path.replace("game.json","assets")).unwrap() {
            let script = script.unwrap();
            if script.file_name().to_str().unwrap().ends_with(".fescript")
            {
                let mut json = String::new();
                fs::File::open(script.path())
                    .expect("Failed to open the JSON file")
                    .read_to_string(&mut json)
                    .expect("Failed to read the JSON file");

                scripts.insert(
                    script.file_name().to_str().unwrap().to_string(),
                    serde_json::from_str::<Vec<Code>>(&json).expect("Failed to deserialize JSON content"));
            }
        }
        settings.office_scripts = scripts;

        settings
    }
}