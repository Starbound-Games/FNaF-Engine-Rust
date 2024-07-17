use crate::GameLoader::Animations;

pub struct OfficeManager
{
    pub curoffice: String,
    pub Office: OfficeData
}

impl OfficeManager {
    pub fn new() -> OfficeManager {
        OfficeManager { curoffice: String::new(), Office: OfficeData::empty()}
    }

    pub fn recache_buttons(ctx: &mut Context, engine: &mut EngineData, event_manager: EventManager, cache: &mut CacheData)
    {
        engine.buttons.clear();
        for object in &engine.game.offices[&engine.officemgr.curoffice].objects {
            let pos_offset: Vec2<f32> =
                Vec2::new(object.position[0] as f32 * 2.13, object.position[1] as f32 * 2.13);

            let binding = &engine.officemgr.Office.sprites[&object.id];
            match object.r#type.as_str() {
                "sprite" if binding.visible && !object.sprite.is_empty() => {
                    let texture = &cache.textures[&object.sprite];
                    engine.buttons.insert(object.id.clone(), Button2D::new(ctx, pos_offset, Option::from(texture), None, "on_sprite_clicked".to_string(), [object.id.to_string()].into()));
                }
                _ => {}
            }
        }
    }


    pub fn update(ctx: &mut Context, state: &mut GameState) -> tetra::Result
    {
        Ok(())
    }

    pub fn init_office(office: String, engine: &mut EngineData, event_manager: & mut EventManager)//, mut scheduler: &Scheduler)
    {
        engine.logger.log("Office Manager","Initializing Office");

        // TODO: Switch audio tracks

        let mut sprites: HashMap<String, Sprite> = HashMap::new();
        let mut animations: HashMap<String, Animation> = HashMap::new();
        let mut doors: HashMap<String, Door> = HashMap::new();
        let mut lights: HashMap<String, Light> = HashMap::new();


        for object in &engine.game.offices[&office].objects
        {
            match object.r#type.as_str()
            {
                ("sprite" |"door_button"|"light_button") =>
                    {
                        sprites.insert(object.id.clone(), Sprite {
                            visible: true,
                            above_panorama: false,
                            hovered: false,
                        });
                    },
                "animation" =>
                    {
                        animations.insert(object.id.clone(), Animation {
                            id: object.id.clone(),
                            is_playing: true,
                            rev: false,
                            animation: vec![],
                        });
                    },
                "door" =>
                    {
                        doors.insert(object.id.clone(), Door {
                            is_closed: false,
                            animation: Vec::new(),
                            button: Button { is_on: false },
                        });
                    },
                "light_button" =>
                    {
                        lights.insert(object.id.clone(), Light {
                            is_on: false,
                        });
                    }
                _ =>
                    {
                        unimplemented!("WHAT THE FUCK IS A {}!!!!!!!!!!!!1 ",object.r#type)
                    }
            }
        }

        engine.officemgr.curoffice = "office".to_string();
        engine.officemgr.Office = OfficeData
        {
            states: engine.game.offices[&office].states.clone(),
            state: "Default".to_string(),
            blinking_effect: false,
            disable_flashlight: false,
            lights,
            doors,
            animations,
            sprites,
        };

        // Script stuff
        event_manager.kill_all_listeners();
        for (path, code) in &engine.game.office_scripts
        {
            println!("Starting Office Script {}", path);
            event_manager.run_script(code);
        }
        event_manager.trigger_event("on_engine_start", &*Vec::new(),engine);
        engine.scene = 1; // This needs to be the last thing we do so engine doesn't try rendering the office before it exists
        event_manager.trigger_event("on_office_start", &*Vec::new(),engine);

    }
}


