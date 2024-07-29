use tetra::input::Key::H;
use crate::GameLoader::Animations;

pub struct OfficeManager
{
    pub curoffice: String,
    pub game_data: GameData,
    pub Loading_Lock: bool,
    pub Loaded: bool,
    pub scroll_x: f32,
    pub hovered_id: String,
}

impl OfficeManager {
    pub fn new() -> OfficeManager {
        OfficeManager {
            curoffice: String::new(),
            game_data: GameData::new("", 0),
            Loading_Lock: false,
            Loaded: false, scroll_x: 0.0,
            hovered_id: String::new()
        }
    }

    pub fn recache_buttons(ctx: &mut Context, engine: &mut EngineData, event_manager: &mut EventManager, cache: &mut CacheData)
    {
        engine.buttons.clear();
        for object in &engine.game.offices[&engine.officemgr.curoffice].objects {
            let pos_offset: Vec2<f32> =
                Vec2::new(object.position[0] as f32 * 2.13, object.position[1] as f32 * 2.13);

            match object.r#type.as_str() {
                "sprite" | "light_button" | "door_button" => { // if engine.officemgr.Office.sprites[&object.id].visible && !object.sprite.is_empty() => {
                    let texture = &cache.textures[&object.sprite];
                    engine.buttons.insert(object.id.clone(), Button2D::new(ctx, pos_offset, Option::from(texture), None, "on_sprite_clicked".to_string(), vec![object.id.to_string()]));
                } /*
                "door_button" => { // if engine.officemgr.Office.sprites[&object.id].visible && !object.sprite.is_empty() => {
                    let texture = &cache.textures[&object.sprite];
                    engine.buttons.insert(object.id.clone(), Button2D::new(ctx, pos_offset, Option::from(texture), None, "on_sprite_clicked".to_string(), vec![object.id.to_string()]));
                }
                "light_button" => { // if engine.officemgr.Office.sprites[&object.id].visible && !object.sprite.is_empty() => {
                    let texture = &cache.textures[&object.sprite];
                    engine.buttons.insert(object.id.clone(), Button2D::new(ctx, pos_offset, Option::from(texture), None, "on_sprite_clicked".to_string(), vec![object.id.to_string()]));
                }*/
                _ => {}
            }
        }
    }


    pub fn update(ctx: &mut Context, state: &mut GameState) -> tetra::Result
    {
        let viewport_width = window::get_width(ctx) as f32;

        let curstate_width = state.cache.textures
            .get(&state.engine.officemgr.game_data.office.states[&state.engine.officemgr.game_data.office.state])
            .map_or(0.0, |curstate| curstate.width() as f32);

        let mouse_position_x = input::get_mouse_position(ctx).x;
        let delta_time = tetra::time::get_delta_time(ctx).as_secs_f32();

        let scroll_speed = if mouse_position_x < viewport_width * 0.1 {
            550.0
        } else if mouse_position_x < viewport_width * 0.225 {
            350.0
        } else if mouse_position_x < viewport_width * 0.35 {
            150.0
        } else if mouse_position_x > viewport_width * 0.9 {
            550.0
        } else if mouse_position_x > viewport_width * 0.775 {
            350.0
        } else if mouse_position_x > viewport_width * 0.65 {
            150.0
        } else {
            0.0
        };

        state.engine.officemgr.scroll_x += scroll_speed * delta_time * (mouse_position_x - viewport_width * 0.5).signum();
        state.engine.officemgr.scroll_x = state.engine.officemgr.scroll_x.clamp(0.0, curstate_width - viewport_width);

        Ok(())
    }

    pub fn init_office(office: String, night: i32, engine: &mut EngineData, event_manager: &mut EventManager) //, mut scheduler: &Scheduler)
    {
        // THIS SHIT IS NOT MULTITHREADED HOW THE HELL IS IT RUNNING 4 TIMES AT THE SAME TIME!?!??!?!??!?!
        if (engine.officemgr.curoffice.is_empty())
        {
            engine.logger.log_error("OfficeManager", "Current Office is null cannot proceed.");
            return;
        }

        if (engine.officemgr.Loading_Lock) {
            engine.logger.log_error("OfficeManager", "Failed to acquire Mutex Lock on office, Is it already Loading?");
            return;
        }

        engine.officemgr.Loading_Lock = true;
        engine.logger.log("Office Manager", "Initializing Office");

        engine.audio.kill_all();
        // TODO: ambience

        let mut sprites: HashMap<String, Sprite> = HashMap::new();
        let mut objects: HashMap<String, Sprite> = HashMap::new();
        let mut animations: HashMap<String, Animation> = HashMap::new();
        let mut doors: HashMap<String, Door> = HashMap::new();
        let mut lights: HashMap<String, Light> = HashMap::new();

        for object in &engine.game.offices[&office].objects
        {
            match object.r#type.as_str()
            {
                "sprite" =>
                    {
                        sprites.insert(object.id.clone(), Sprite {
                            visible: true,
                            above_panorama: false,
                            hovered: false,
                        });
                        objects.insert(object.id.clone(), Sprite {
                            visible: true,
                            above_panorama: false,
                            hovered: false,
                        });
                    },
                "door_button" =>
                    {
                        objects.insert(object.id.clone(), Sprite {
                            visible: true,
                            above_panorama: false,
                            hovered: false,
                        });
                    }
                "animation" =>
                    {
                        animations.insert(object.id.clone(), Animation {
                            id: object.animation.clone(),
                            is_playing: true,
                            is_reversed: false
                        });
                        objects.insert(object.id.clone(), Sprite {
                            visible: true,
                            above_panorama: false,
                            hovered: false,
                        });
                    },
                "door" =>
                    {
                        doors.insert(object.id.clone(), Door {
                            animation: object.animation.clone(),
                            is_closed: false,
                            button: Button {
                                is_on: false,
                                clickable: true
                            }
                        });
                        objects.insert(object.id.clone(), Sprite {
                            visible: true,
                            above_panorama: false,
                            hovered: false,
                        });
                    },
                "light_button" =>
                    {
                        lights.insert(object.id.clone(), Light {
                            is_on: false,
                            clickable: false,
                        });
                        objects.insert(object.id.clone(), Sprite {
                            visible: true,
                            above_panorama: false,
                            hovered: false,
                        });
                    }
                _ =>
                    {
                        engine.logger.log_error("OfficeManager", "Unknown Object Type: {:?}");
                    }
            }
        }
        engine.officemgr.hovered_id = String::new();
        engine.officemgr.game_data.night = night;
        engine.officemgr.curoffice = office.to_string();
        engine.officemgr.game_data.office.states = engine.game.offices[&office].states.clone();
        engine.officemgr.game_data.office.sprites = sprites;
        engine.officemgr.game_data.office.animations = animations;
        engine.officemgr.game_data.office.lights = lights;
        engine.officemgr.game_data.office.doors = doors;
        engine.officemgr.game_data.office.objects = objects;


        // Script stuff
        event_manager.kill_all_listeners();
        for (path, code) in &engine.game.office_scripts
        {
            println!("Starting Office Script {}", path);
            event_manager.run_script(code);
        }
        engine.clock.clone().stop();
        engine.clock.clone().reset();
        engine.clock.clone().start();
        event_manager.trigger_event("on_engine_start", &*Vec::new(), engine);
        engine.scene = 1; // This needs to be the last thing we do so engine doesn't try rendering the office before it exists
        engine.needs_recache = true;
        event_manager.trigger_event("on_night_start", &*Vec::new(), engine);
        engine.officemgr.Loading_Lock = false;
    }
}


