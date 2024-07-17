extern crate find_folder;
extern crate serde;
extern crate serde_json;
extern crate tetra;

use std::collections::HashMap;
use std::env;
use std::fmt::Debug;
use std::hash::Hash;
use std::path::{Path, PathBuf};
use std::string::String;
use std::time::Instant;
use tetra::{Context, ContextBuilder, State, TetraError};
use tetra::time::Timestep;

use GameLoader::Game;
use crate::GameLoader::Office;

include!("Utils/Logger.rs");
include!("Loaders/GameLoader.rs");
include!("Utils/SmartFPS.rs");
include!("Loaders/AssetLoader.rs");
include!("Renderers/MenuRenderer.rs");
//include!("Loaders/FontLoader.rs");
include!("Logic/Game/MenuManager.rs");
include!("Logic/Scripting/EventManager.rs");
include!("Logic/Game/OfficeManager.rs");
include!("Renderers/OfficeRenderer.rs");
include!("Logic/Game/Types/OfficeData.rs");
include!("Logic/Game/Types/Button.rs");



//use bevy::prelude::*;
//use bevy::sprite::Anchor;
//use bevy::transform;
//extern crate Nylon;
//use wasm_bindgen::prelude::*;
/*use Nylon::*;



struct Engine;

impl Nylon::Game for Engine {
    fn initialize(&mut self, context: &mut Nylon::Context) -> Result<(), Error> {
        Ok(())
    }

    fn update(&mut self, context: &mut Nylon::Context) -> Result<(), Error> {
        Ok(())
    }

    fn draw(&mut self, context: &mut Nylon::Context) -> Result<(), Error> {
        clear(context, Color::rgb(100, 149, 237));
        Ok(())
    }
}


pub fn main() {
    unsafe {
    Nylon::run(Box::new(Engine)).unwrap();
}}
*/
/*
fn main() {
    env::set_var("BEVY_ASSET_ROOT", "G:\\actually repos\\FNaF-Engine-Rust\\target\\debug");
    App::new()

        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut curmenu = "Main";


    // Init startup here
    let logger = Logger::new();
    logger.draw_splash();

    // let assets = AssetLoader::load_assets(ctx, &mut curmenu, &logger)?;

    let mut fps = SmartFPS::new(5);
    let mut stopwatch: Instant = Instant::now();

    let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("target\\debug\\assets").unwrap();
    let mut textures: HashMap<String, Handle<Image>> = HashMap::new();
    let mut fonts: HashMap<i32, Font> = HashMap::new();

    // Load Game
    let game = GameLoader::Load("G:\\actually repos\\FNaF-Engine-Rust\\target\\debug\\assets\\game.json");

    for (k, v) in &game.menus {
        logger.log("Main", k);
    }
    let current_dir = std::env::current_dir().unwrap();


    // Init Engine
    if !game.menus[curmenu].properties.BackgroundImage.is_empty() {
        let mut path = assets
            .join("sprites")
            .join(&game.menus[curmenu].properties.BackgroundImage.replace("\\", "\\"))
            .to_str()
            .unwrap()
            .to_string();

        println!("{}", &path);

        if Path::new(&path).exists() {
            textures.insert(
                String::from(&game.menus[curmenu].properties.BackgroundImage),
                asset_server.load(format!("sprites\\{}", &game.menus[curmenu].properties.BackgroundImage)),
            );
        }
    }
    for (thing, thinf) in &textures
    {
        println!("{} {}", thing, thinf.id());
    }
    for element in &game.menus[curmenu].elements {
        if !element.sprite.is_empty() {
            let mut path = assets
                .join("sprites")
                .join(&element.sprite.replace("\\", "\\"))
                .to_str()
                .unwrap()
                .to_string();
            println!("{}", &path);

            if Path::new(&path).exists() {
                textures.insert(String::from(&element.sprite), asset_server.load(format!("sprites\\{}", &element.sprite)));
            }
        }
    }
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        texture: textures[&game.menus["Main"].properties.BackgroundImage].clone(),
        ..default()
    });

    for element in &game.menus[curmenu].elements {
        if !element.sprite.is_empty() {
            commands.spawn(SpriteBundle {
                texture: textures[&element.sprite].clone(),
                transform: Transform::from_xyz(element.x as f32*2.32-630.0, element.y as f32*-2.32+360.0, 1.0),

                ..default()
            });
        }
    }
}*/

pub struct CacheData {
    textures: HashMap<String, Texture>,
    fonts: HashMap<i32, Font>,
    texts: HashMap<String, Text>
}

pub struct EngineData {
    fps: SmartFPS,
    stopwatch: Instant,
    game: Game,
    assets: PathBuf,
    scene: i32,
    pub menumgr: MenuManager,
    pub officemgr: OfficeManager,
    pub logger: Logger,
    pub buttons: HashMap<String, Button2D>
}

pub struct GameState {
    cache: CacheData,
    engine: EngineData,
    eventmanager: EventManager
}

impl CacheData {
    fn new(ctx: &mut Context, mut engine_data: &mut EngineData) -> tetra::Result<CacheData> {

        if (engine_data.game.menus.contains_key("Warning"))
        {
            engine_data.menumgr.curmenu = "Warning".parse().unwrap();
        }
        else {
            engine_data.menumgr.curmenu = "Main".parse().unwrap()
        }
        let mut menus_cache = AssetLoader::load_menus(ctx, &engine_data.game, &engine_data.menumgr, &engine_data.assets, &engine_data.logger)?;
        let offices_cache = AssetLoader::load_offices(ctx, &engine_data.game, &engine_data.assets, &engine_data.logger)?;

        // Thanks to the way rust works, doing it like this makes the data
        // in OfficeCache move instead of clone, into MenuCache
        menus_cache.fonts.extend(offices_cache.fonts);
        menus_cache.textures.extend(offices_cache.textures);

        Ok(CacheData {
            textures: menus_cache.textures,
            fonts: menus_cache.fonts,
            texts: HashMap::new(),
        })
    }
}

impl EngineData {
    fn new() -> tetra::Result<EngineData> {
        let mut menumgr: MenuManager = MenuManager::new();
        let mut officemgr: OfficeManager = OfficeManager::new();

        // Init startup here
        let logger = Logger::new();
        logger.draw_splash();

        // Load Game
        let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("target/debug/assets").unwrap();
        let game = GameLoader::Load(&assets.join("game.json").to_str().unwrap());

        let mut fps = SmartFPS::new(5);
        let mut stopwatch: Instant = Instant::now();

        Ok(EngineData {
            fps,
            stopwatch,
            game,
            assets,
            scene: 0,
            menumgr,
            officemgr,
            logger,
            buttons: HashMap::new(),
        })
    }
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let mut engine = EngineData::new().unwrap();
        let mut cache = CacheData::new(ctx, &mut engine).unwrap();
        let mut eventmanager = EventManager::new(engine.logger.clone());
        eventmanager.run_script(&engine.game.menus[&engine.menumgr.curmenu].code);
        eventmanager.trigger_event("current_tick_equals", &["10".parse().unwrap()], &mut engine);


       // OfficeManager::init_office("office".to_string(), &mut engine, &mut eventmanager); // this is a hardcoded night start
        MenuManager::recache_buttons(ctx, &mut engine, &mut eventmanager, &mut cache);

        Ok(GameState {
            cache,
            engine,
            eventmanager
        })
    }
}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        match (self.engine.scene)
        {
            0 => {
                MenuRenderer::render(ctx, &mut self.engine, &mut self.cache)?;
            },
            1 => {
                OfficeRenderer::render(ctx, &mut self.engine, &mut self.cache)?;
            }
            _ => {}
        }

        Ok(())
    }

    fn update(&mut self, ctx: &mut Context) -> Result<(), TetraError> {
        self.eventmanager.trigger_event("on_game_loop", &[String::new()], &mut self.engine);

        let mut event_name: String = String::new();
        let mut event_args: Vec<String> = Vec::new();

        for (k, mut button) in &mut self.engine.buttons {
            button.update(ctx);
            if button.is_clicked {
                event_name = button.event.clone();
                event_args = button.event_args.clone();
            }
        }
        self.eventmanager.trigger_event(&event_name, &event_args, &mut self.engine);

        match (self.engine.scene)
        {
            0 => {
                MenuManager::update(ctx, self)?;
            },
            1 => {
                OfficeManager::update(ctx, self)?;
            }
            _ => {}
        }

        Ok(())
    }
}

fn main() -> tetra::Result {
    ContextBuilder::new("FNaF Engine: Rust", 1280, 720)
        .fps_limit(false)
        .show_mouse(true)
        .vsync(false)
        .timestep(Timestep::Variable)
        .build()?
        .run(GameState::new)
}