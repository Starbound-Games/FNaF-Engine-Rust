extern crate find_folder;
extern crate serde;
extern crate serde_json;
extern crate tetra;
extern crate rayon;
use crate::rayon::iter::ParallelIterator;

use std::collections::HashMap;
use std::env;
use std::fmt::Debug;
use std::future::IntoFuture;
use std::hash::Hash;
use std::path::{Path, PathBuf};
use std::string::String;
use std::time::Instant;
use tetra::input::Key::P;
use tetra::time::Timestep;
use tetra::{audio, Context, ContextBuilder, Event, State, TetraError};
use tetra::input::is_mouse_button_down;
use tetra::window::{set_fullscreen, set_mouse_visible};
use crate::GameLoader::Office;
use GameLoader::Game;
use tetra::graphics::animation;
use tetra::graphics::scaling::{ScalingMode, ScreenScaler};

include!("Utils/Logger.rs");
include!("Loaders/GameLoader.rs");
include!("Utils/SmartFPS.rs");
include!("Loaders/AssetLoader.rs");
include!("Loaders/PluginManager.rs");
include!("Renderers/MenuRenderer.rs");
//include!("Loaders/FontLoader.rs");
include!("Logic/Game/MenuManager.rs");
include!("Logic/Scripting/EventManager.rs");
include!("Logic/Scripting/LuaBindings.rs");
include!("Logic/Game/OfficeManager.rs");
include!("Renderers/OfficeRenderer.rs");
include!("Logic/Game/Types/OfficeData.rs");
include!("Logic/Game/Types/Button.rs");
include!("Logic/Game/TickManager.rs");
include!("Logic/Game/AudioManager.rs");
include!("Utils/CrashHandler.rs");
include!("Logic/Game/Types/RevAnimation.rs");
pub struct CacheData {
    textures: HashMap<String, Texture>,
    animations: HashMap<String, RevAnimation>,
    fonts: HashMap<i32, Font>,
    texts: HashMap<String, Text>,
}

pub struct GameState {
    screen_scaler: ScreenScaler,
    cache: CacheData,
    engine: EngineData,
    eventmanager: EventManager,
    firstrun:bool,
    plugin_manager: PluginManager
}


pub struct EngineData {
    pub fps: SmartFPS,
    pub stopwatch: Instant,
    pub game: Game,
    pub assets: PathBuf,
    pub scene: i32,
    pub menumgr: MenuManager,
    pub officemgr: OfficeManager,
    pub logger: Logger,
    pub buttons: HashMap<String, Button2D>,
    pub clock: Arc<TickManager>,
    pub audio: AudioManager,
    pub fullscreen: bool,
    pub show_mouse: bool,
    pub update_context: bool,
    pub hovered: bool,
    pub disabled_clicks: Vec<bool>,
    pub needs_recache: bool,
    pub camera: tetra::graphics::Camera,
}

impl CacheData {
    fn new(ctx: &mut Context, mut engine_data: &mut EngineData) -> tetra::Result<CacheData> {
        if (engine_data.game.menus.contains_key("Warning")) {
            engine_data.menumgr.curmenu = "Warning".parse().unwrap();
        } else {
            engine_data.menumgr.curmenu = "Main".parse().unwrap()
        }
        let mut menus_cache = AssetLoader::load_menus(
            ctx,
            &engine_data.game,
            &engine_data.assets,
            &engine_data.logger,
        )?;
        let offices_cache = AssetLoader::load_offices(
            ctx,
            &engine_data.game,
            &engine_data.assets,
            &engine_data.logger,
        )?;
        let anim_cache = AssetLoader::load_animations(
            ctx,
            &engine_data.assets,
            &engine_data.logger,
            &engine_data.game,
        )?;
        engine_data.logger.log("AssetLoader", "Finished caching assets.");

        // Thanks to the way rust works, doing it like this makes the data
        // in OfficeCache move instead of clone, into MenuCache
        menus_cache.fonts.extend(offices_cache.fonts);
        menus_cache.textures.extend(offices_cache.textures);

        Ok(CacheData {
            textures: menus_cache.textures,
            animations: anim_cache.animations,
            fonts: menus_cache.fonts,
            texts: HashMap::new(),
        })
    }
}

impl EngineData {
    pub fn new(ctx: &mut Context) -> tetra::Result<EngineData> {
        let menumgr = MenuManager::new();
        let officemgr = OfficeManager::new();
        let clock = Arc::new(TickManager::new());
        let mut audio = AudioManager::new();

        let logger = Logger::new();
        logger.draw_splash();

        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("target/debug/assets")
            .unwrap();
        let game = GameLoader::Load(&assets.join("game.json").to_str().unwrap());

        let fps = SmartFPS::new(5);
        let stopwatch = Instant::now();

        let disabled_clicks = vec![false, false, false, false];

        audio.load_audio_assets(ctx, &assets);

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
            clock,
            audio,
            fullscreen: false,
            show_mouse: true,
            update_context: false,
            hovered: false,
            disabled_clicks,
            needs_recache: false,
            camera: graphics::Camera::new(1280.0, 720.0),

        })
    }
}

impl GameState {
    pub fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let scaler = ScreenScaler::with_window_size(ctx, 1280, 720, ScalingMode::ShowAllPixelPerfect)?;
        let mut engine = EngineData::new(ctx)?;
        let mut cache = CacheData::new(ctx, &mut engine)?;
        let mut eventmanager = EventManager::new(engine.logger.clone());

        eventmanager.run_script(&engine.game.menus[&engine.menumgr.curmenu].code);
        MenuManager::recache_buttons(ctx, &mut engine, &mut eventmanager, &mut cache);

        engine.clock.clone().start();

        Ok(GameState {
            screen_scaler: scaler,
            cache,
            engine,
            eventmanager,
            firstrun: true,
            plugin_manager: PluginManager::new()
        })
    }
}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::set_canvas(ctx, self.screen_scaler.canvas());
        match self.engine.scene {
            0 => {
                if !self.engine.needs_recache {
                    MenuRenderer::render(ctx, &mut self.engine, &mut self.cache)?;
                }
            }
            1 => {
                OfficeRenderer::render(ctx, &mut self.engine, &mut self.cache, tetra::time::get_delta_time(ctx))?;
            }
            _ => {}
        }

        graphics::reset_canvas(ctx);
        graphics::clear(ctx, Color::BLACK);

        self.screen_scaler.draw(ctx);
        Ok(())
    }

    fn update(&mut self, ctx: &mut Context) -> Result<(), TetraError> {
       // let firstrun = self.firstrun.clone();
      //  if firstrun {
       //     let mut plugin_manager = PluginManager::new();
        //    plugin_manager.load_plugins();
        //    plugin_manager.initialize_plugins();
        //    self.firstrun = false
    //    }
        self.plugin_manager.update_plugins(ctx);
        if self.engine.needs_recache {
            self.engine.needs_recache = false;
            match self.engine.scene {
                0 => {
                    MenuManager::recache_buttons(
                        ctx,
                        &mut self.engine,
                        &mut self.eventmanager,
                        &mut self.cache,
                    );
                }
                1 => {
                    OfficeManager::recache_buttons(
                        ctx,
                        &mut self.engine,
                        &mut self.eventmanager,
                        &mut self.cache,
                    );
                }
                _ => {}
            }
        }

        if *self.engine.clock.ticked.lock().unwrap() {
            ScriptingAPI::TickEvents(&mut self.engine, &mut self.eventmanager, ctx).expect("FATAL: Failed to Tick Events.");
            *self.engine.clock.ticked.lock().unwrap() = false;
        }

        if self.engine.update_context {
            set_mouse_visible(ctx, self.engine.show_mouse).expect("Error: Failed to set mouse visibility.");
            set_fullscreen(ctx, self.engine.fullscreen).expect("Error: failed to set fullscreen.");
            self.engine.update_context = false;
        }

        match self.engine.scene {
            0 => {
                MenuManager::update(ctx, self)?;
            }
            1 => {
                OfficeManager::update(ctx, self)?;
            }
            _ => {}
        }

        //plugin_manager.update_plugins(ctx, self);

        Ok(())
    }

    fn event(&mut self, _: &mut Context, event: Event) -> tetra::Result {
        if let Event::Resized { width, height } = event {
           // self.screen_scaler.set_outer_size(width, height);
            self.engine.needs_recache = true;
        }

        Ok(())
    }
}

#[tokio::main]
async fn main() -> tetra::Result {
    //let crashhandler = CrashHandler::new();
    let mut plugin_manager = PluginManager::new();
    unsafe {
        plugin_manager.load_plugin("G:\\actually repos\\FNaF-Engine-Rust\\src\\PluginAPI\\Builtin-Plugins\\FNaF-World-Plugin\\target\\debug\\FNaF_World_Plugin.dll");
    }
    plugin_manager.initialize_plugins();
    ContextBuilder::new("FNaF Engine: Rust", 1280, 720)
        .fps_limit(false)
        .show_mouse(true)
        .vsync(false)
        .timestep(Timestep::Variable)
        .high_dpi(true)
        .build()?
        .run(|ctx| {
            let mut game_state = GameState::new(ctx)?;

            game_state.plugin_manager = plugin_manager;
            Ok(game_state)
        })
}
