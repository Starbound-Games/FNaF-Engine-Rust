use crate::GameLoader::CamUI;

pub struct OfficeData {
    pub states: HashMap<String, String>,
    pub state: String,
    pub blinking_effect: bool,
    pub disable_flashlight: bool,
    pub objects: HashMap<String, Sprite>,
    pub lights: HashMap<String, Light>,
    pub doors: HashMap<String, Door>,
    pub animations: HashMap<String, Animation>,
    pub sprites: HashMap<String, Sprite>,
}

impl OfficeData {
    pub fn empty() -> OfficeData
    {
        OfficeData {
            states: HashMap::new(),
            state: "Default".to_string(),
            blinking_effect: false,
            disable_flashlight: false,
            objects: HashMap::new(),
            lights: HashMap::new(),
            doors: HashMap::new(),
            animations: HashMap::new(),
            sprites: HashMap::new(),
        }
    }
    pub fn set_state(&mut self, state: String) {
        self.state = state;
    }
}

pub struct Sprite {
    pub visible: bool,
    pub above_panorama: bool,
    pub hovered: bool,
}

pub struct Animation {
    pub id: String,
    pub is_playing: bool,
    pub is_reversed: bool
}

pub struct Door {
    pub is_closed: bool,
    pub button: Button,
    pub animation: String
}

pub struct Button {
    pub is_on: bool,
    pub clickable: bool
}

pub struct Light {
    pub is_on: bool,
    pub clickable: bool
}

pub struct Settings {
    pub mask: bool,
    pub flashlight: bool,
    pub toxic: bool,
    pub panorama: bool,
    pub power: bool,
}

pub struct Power {
    pub level: i32,
    pub usage: i32,
    pub enabled: bool,
    pub ucn: bool,
    pub power_out_animation: String,
    pub animatronic_jumpscare: String,
}

pub struct Camera {
    pub panorama: bool,
    pub static_camera: bool,
    pub states: HashMap<String, String>,
    pub state: String,
    pub scroll: i32,
}

impl Camera {
    pub fn set_state(&mut self, state: String) {
        self.state = state;
    }
}

pub struct Animatronic {
    pub ignores_mask: bool,
    pub path: Vec<PathNode>,
    pub ai: Vec<i32>,
    pub jumpscare: AnimatronicJumpscare,
    pub location: PathNode,
    pub name: String,
    pub move_time: i32,
    pub state: String,
    pub location_index: i32,
    pub phantom: bool,
}
pub struct AnimatronicJumpscare {
    pub sound: String,
    pub jumpscare: String,
    pub offset: i32
}

pub struct GameData {
    pub path: String,
    pub night: i32,
    pub office: OfficeData,
    pub settings: Settings,
    pub player: Player,
    // pub Animations: Vec<String, AnimationFrame>, //old animation data structure got deprecated, code a converter later
    pub cameras: HashMap<String, Camera>,
    pub cam_ui: CamUI,
    pub animatronics: HashMap<String, Animatronic>,
    pub power: Power,
    pub time: i32,
    pub paused: bool
}

impl GameData {
    pub fn new(path: &str, night: i32) -> GameData {
        GameData {
            path: String::from(path),
            night,
            settings: Settings {
                mask: true,
                flashlight: true,
                toxic: true,
                panorama: true,
                power: false,
            },
            office: OfficeData::empty(),
            player: Player::new(),
            cameras: Default::default(),
            cam_ui: CamUI {
                buttons: HashMap::new(),
                music_box: vec![],
                sprites: HashMap::new(),
            },
            animatronics: Default::default(),
            power: Power {
                level: 100,
                usage: 0,
                enabled: true,
                ucn: false,
                power_out_animation: String::new(),
                animatronic_jumpscare: String::new(),
            },
            time: 0,
            paused: false
        }
    }

    pub fn set_time(&mut self, time: i32) {
        self.time = time;
    }

    pub fn quit(&mut self) {
        // deprecated
    }

    pub fn disable_time(&mut self) {
        self.paused = true;
    }
    pub fn enable_time(&mut self) {
        self.paused = false;
    }

    pub fn goto_menu(&mut self) {
        // deprecated
    }
}


pub struct Player {
    pub is_camera_up: bool,
    pub camera_button_toggle: bool,
    pub is_mask_on: bool,
    pub is_flashlight_on: bool,
    pub current_camera: String,
    pub signal_interrupted: bool,
}

impl Player {
    pub fn new() -> Player {
        Player {
            is_camera_up: false,
            camera_button_toggle: false,
            is_mask_on: false,
            is_flashlight_on: false,
            current_camera: String::new(),
            signal_interrupted: false,
        }
    }

    pub fn set_camera(&mut self, cam: String) {
        self.current_camera = cam;
    }

    pub fn putdown(&mut self) {
        self.is_camera_up = false;
    }

    pub fn pullup(&mut self) {
        self.is_camera_up = true;
    }

    pub fn mask_off(&mut self) {
        self.is_mask_on = false;
    }

    pub fn mask_on(&mut self) {
        self.is_mask_on = true;
    }
}

pub struct PathNode {
    pub id: String,
    pub type_: String,
    pub chance: i32,
    pub state: String,
    pub path: Vec<PathNode>,
    pub camid: String,
}

impl PathNode {
    pub fn new(id: String, type_: String, chance: i32, state: String, path: Vec<PathNode>, camid: String) -> Self {
        Self { id, type_, chance, state, path, camid }
    }
}

impl ToString for PathNode {
    fn to_string(&self) -> String {
        format!("ID: {} TYPE: {} CHANCE: {} STATE: {}", self.id, self.type_, self.chance, self.state)
    }
}
