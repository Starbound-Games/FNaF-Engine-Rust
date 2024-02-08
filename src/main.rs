extern crate find_folder;
extern crate serde;
extern crate serde_json;
extern crate tetra;

//use bevy::render::texture::Image;
use std::collections::HashMap;
use std::env;
use std::fmt::Debug;
use std::path::{Path, PathBuf};
use std::string::String;
use std::time::Instant;
//use bevy::math::{vec2, vec3};

use tetra::{Context, ContextBuilder, State};
use tetra::time::Timestep;

use GameLoader::Game;

include!("Utils/Logger.rs");
include!("Loaders/GameLoader.rs");
include!("Utils/SmartFPS.rs");
include!("Loaders/AssetLoader.rs");
include!("Renderers/MenuRenderer.rs");
//include!("Loaders/FontLoader.rs");
include!("Logic/Game/MenuManager.rs");

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

pub struct GameState {
    textures: HashMap<String, Texture>,
    fonts: HashMap<i32, Font>,
    texts: HashMap<String, Text>,
    fps: SmartFPS,
    stopwatch: Instant,
    game: Game,
    in_menus: bool,
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let mut curmenu = "Main";

        // Init startup here
        let logger = Logger::new();
        logger.draw_splash();

        let assets = AssetLoader::load_assets(ctx, &mut curmenu, &logger)?;

        let mut fps = SmartFPS::new(5);
        let mut stopwatch: Instant = Instant::now();

       // let font = FontLoader::font_from_name(ctx, "Arial", 16).unwrap();
       // println!("{:?}", );

        Ok(GameState {
            textures: assets.textures,
            fonts: assets.fonts,
            texts: HashMap::new(),
            fps,
            stopwatch,
            game: assets.game,
            in_menus: true,
        })
    }
}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        if self.in_menus {
            MenuRenderer::render(ctx, self)?;
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