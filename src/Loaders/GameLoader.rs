
use std::fs::FileType;
use regex::Replacer;
use rodio::source::from_iter;

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
    use serde_json::Map;
    use std::fmt::Display;

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
        #[serde(skip_deserializing)]
        pub(crate) animations: HashMap<String, Vec<AnimationJson>>,
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
        #[serde(default)]
        pub buttons: HashMap<String, Vec<MultiType>>,
        #[serde(default)]
        pub music_box: Vec<i32>,
        #[serde(default)]
        pub sprites: HashMap<String, Vec<MultiType>>,
    }

    #[derive(Debug,  Serialize, Deserialize, Clone)]
    #[serde(untagged)]
    pub enum MultiType {
        Str(String),
        Int(i32),
    }

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

    struct StringError(String);

    impl<E> From<E> for StringError
    where
        E: Display,
    {
        fn from(value: E) -> Self {
            Self(value.to_string())
        }
    }


    fn ignore_if_map<'de, D>(deserializer: D) -> Result<Vec<Element>, D::Error>
        where
            D: serde::Deserializer<'de>,
    {
        let value: Value = Deserialize::deserialize(deserializer)?;

        match &value {
            Value::Array(_array) => {
                let elements: Result<Vec<Element>, _> = serde_json::from_value(value.clone());
                elements.map_err(|e| D::Error::custom(format!("Failed to deserialize elements: {}", e)))
            }
            Value::Object(_hashmap) => {
                let parse: HashMap<String, Value> = serde_json::from_value(value.clone()).unwrap();
                let values_vec = parse.values().cloned();
                let mut elements_vec: Vec<Element> = Vec::new();
                let mut i = 0;

                for val in values_vec.skip(1) {
                     match serde_json::from_value(val).map_err(|e| e.to_string()) {
                        Ok(n) => {
                            elements_vec.insert(i, n);
                            i += 1;
                        },
                        Err(err) => println!("INVALID ELEMENT: {}", err),
                    }
                }
                Ok(elements_vec)
            }
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
        pub animation: String,
        #[serde(default)]
        pub text: String,
        pub r#type: String,
        #[serde(default)]
        pub(crate) sprite: String,
        #[serde(default)]
        pub animatronic: String,
        #[serde(default)]
        pub x: i32,
        #[serde(default)]
        pub y: i32,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Properties {
        #[serde(default)]
        pub(crate) BackgroundImage: String,
        #[serde(default)]
        pub BackgroundColor: String,
        #[serde(default)]
        pub BackgroundMusic: String,
        #[serde(default)]
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
        pub camera: CameraPanel,
        pub mask: MaskPanel,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct CameraPanel {
        pub image: String,
        pub position: Vec<i32>,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct MaskPanel {
        pub image: String,
        pub position: Vec<i32>,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Sounds {
        pub ambience: String,
        pub animatronic_move: Vec<String>,
        pub blip: String,
        pub camdown: String,
        pub camup: String,
        pub flashlight: String,
        pub maskbreathing: String,
        pub maskoff: String,
        pub maskon: String,
        pub masktoxic: String,
        pub music_box_run_out: String,
        pub phone_calls: Vec<String>,
        pub powerout: String,
        pub signal_interrupted: String,
        pub stare: String,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct AnimationJson {
        pub duration: i32,
        pub sprite: String,
    }

    pub fn Load(input_json_path: &str) -> Game {
        let mut file = std::fs::File::open(input_json_path).expect("Failed to open the JSON file");
        let mut content = String::new();
        file.read_to_string(&mut content)
            .expect("Failed to read the JSON file");

        let mut settings = serde_json::from_str::<Game>(&content)
            .expect("Failed to deserialize JSON content");

        let mut scripts: HashMap<String, Vec<Code>> = HashMap::new();
        let mut animations: HashMap<String, Vec<AnimationJson>> = HashMap::new();

        for script in fs::read_dir(input_json_path.replace("game.json","scripts")).unwrap() {
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

        for script in fs::read_dir(input_json_path.replace("game.json","animations")).unwrap() {
            let animation = script.unwrap();
            if animation.file_name().to_str().unwrap().ends_with(".json")
            {
                let mut json = String::new();
                fs::File::open(animation.path())
                    .expect("Failed to open the JSON file")
                    .read_to_string(&mut json)
                    .expect("Failed to read the JSON file");

                animations.insert(
                    animation.file_name().to_str().unwrap().to_string(),
                    serde_json::from_str(&json).expect("Failed to deserialize JSON content"));
            }
        }
        settings.animations = animations;

        settings
    }
}