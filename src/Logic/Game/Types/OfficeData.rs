pub struct OfficeData {
    pub states: HashMap<String, String>,
    pub state: String,
    pub blinking_effect: bool,
    pub disable_flashlight: bool,
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
            state: "".to_string(),
            blinking_effect: false,
            disable_flashlight: false,
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
    pub rev: bool,
    pub animation: Vec<Frame>,
}

pub struct Door {
    pub is_closed: bool,
    pub animation: Vec<Frame>,
    pub button: Button,
}

pub struct Button {
    pub is_on: bool,
}

pub struct Light {
    pub is_on: bool,
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
    pub power_out_animation: PowerOutAnim,
    pub animatronic_jumpscare: String,
}

pub struct PowerOutAnim {
    pub frames: Vec<Frame>,
    pub offset: i32,
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
    pub offset: i32,
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

pub struct Frame {
    pub duration: i32,
    pub sprite: String,
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
