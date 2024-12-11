use std::cell::RefCell;
use std::fmt::Pointer;
use std::rc::Rc;
use std::borrow::BorrowMut;
use std::ops::Deref;
use crate::GameLoader::Element;

#[derive(Clone)]
pub struct MenuManager
{
    curmenu: String,
    arrows_enabled: bool,
    selected_button_id: String
}

impl MenuManager {
    pub fn new() -> MenuManager {
        let curmenu = String::new();
        MenuManager { curmenu,  arrows_enabled: true, selected_button_id: String::new()}
    }

    pub fn recache_buttons(ctx: &mut Context, engine: &mut EngineData, event_manager: &mut EventManager, cache: &mut CacheData)
    {
        engine.buttons.clear();
        let x_scale: f32 = (window::get_width(ctx) / 1280) as f32;
        let y_scale: f32 = (window::get_height(ctx)  / 720) as f32;
        for element in &engine.game.menus[&engine.menumgr.curmenu].elements {
            let pos_offset: Vec2<f32> =
                Vec2::new(element.x as f32 * 2.15 * x_scale, element.y as f32 * 2.15 * y_scale);

            match element.r#type.as_str() {
                ("Button") if !engine.buttons.contains_key(&element.id) && !element.hidden  => {
                    let uid = format!("{}-{}", &element.text, &element.fontsize);

                    let text = cache.texts
                        .entry(uid.clone())
                        .or_insert_with(|| Text::new(&element.text, cache.fonts[&element.fontsize].clone()));

                    engine.buttons.insert(element.id.clone(), Button2D::new(ctx, pos_offset, None, Some(text), "button_clicked".to_string(), vec![element.id.to_string()]));
                }
                "Image" if !element.hidden && !element.sprite.is_empty() => {
                    let texture = &cache.textures[&element.sprite];
                    engine.buttons.insert(element.id.clone(), Button2D::new(ctx, pos_offset, Option::from(texture), None, "image_clicked".to_string(), vec![element.id.to_string()]));

                }
                _ => {}
            }
        }
    }

    pub fn update(ctx: &mut Context, state: &mut GameState) -> tetra::Result
    {
        Ok(())
    }

    pub fn is_button_selected(mut element_id: String, engine: &mut EngineData) -> bool
    {
        let element_option = engine.game.menus.get_mut(&engine.menumgr.curmenu).unwrap().elements.iter_mut().find(|x| x.id == element_id);
        if let Some(element) = element_option {
            let button = &engine.buttons.get(&element.id);
            if button.is_some() {
                return button.expect("Error: Tried to access unloaded element.").is_hovered;
            }
        }
        return false
    }

    pub fn set_background(mut path: &str, engine: &mut EngineData)
    {
        engine.game.menus.get_mut(&engine.menumgr.curmenu).unwrap().properties.BackgroundImage = path.to_string();
    }

    pub fn set_element_text(mut element_id: &str, data: String, engine: &mut EngineData)
    {
        let element_option = engine.game.menus.get_mut(&engine.menumgr.curmenu).unwrap().elements.iter_mut().find(|x| x.id == element_id);
        if let Some(element) = element_option {
            element.text = data;
        }
    }

    pub fn set_element_sprite(mut element_id: &str, data: String, engine: &mut EngineData)
    {
        let element_option = engine.game.menus.get_mut(&engine.menumgr.curmenu).unwrap().elements.iter_mut().find(|x| x.id == element_id);
        if let Some(element) = element_option {
            element.sprite = data;
        }
    }

    pub fn show_element(mut element_id: &str, engine: &mut EngineData)
    {
        let element_option = engine.game.menus.get_mut(&engine.menumgr.curmenu).unwrap().elements.iter_mut().find(|x| x.id == element_id);
        if let Some(element) = element_option {
            element.hidden = false;
        }
    }

    pub fn hide_element(mut element_id: &str, engine: &mut EngineData)
    {
        let element_option = engine.game.menus.get_mut(&engine.menumgr.curmenu).unwrap().elements.iter_mut().find(|x| x.id == element_id);
        if let Some(element) = element_option {
            element.hidden = true;
        }
    }

    pub fn goto_menu(menu: &str, engine: &mut EngineData, event_manager: & mut EventManager)
    {
        engine.logger.log("Menu Manager",format!("Going to: {}", menu).as_str());
        engine.audio.kill_all();
        engine.needs_recache = true;
        engine.menumgr.curmenu = menu.to_string();
        engine.audio.play(&engine.game.menus[menu].properties.BackgroundMusic, true);
        event_manager.kill_all_listeners();
        engine.scene = 0;
        event_manager.run_script(&engine.game.menus[menu].code);
        event_manager.trigger_event("on_menu_start", &*Vec::new(),engine);
        engine.clock.clone().stop();
        engine.clock.clone().reset();
        engine.clock.clone().start();

    }
}


