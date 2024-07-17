use std::cell::RefCell;
use std::fmt::Pointer;
use std::rc::Rc;
use std::borrow::BorrowMut;
use std::ops::Deref;

#[derive(Clone)]
pub struct MenuManager
{
    curmenu: String,
}

impl MenuManager {
    pub fn new() -> MenuManager {
        let curmenu = String::new();
        MenuManager { curmenu }
    }

    pub fn recache_buttons(ctx: &mut Context, engine: &mut EngineData, event_manager: &mut EventManager, cache: &mut CacheData)
    {
        engine.buttons.clear();
        for element in &engine.game.menus[&engine.menumgr.curmenu].elements {
            let pos_offset: Vec2<f32> =
                Vec2::new(element.x as f32 * 2.15, element.y as f32 * 2.15);

            match element.r#type.as_str() {
                ("Button") if !element.hidden => {
                    let uid = format!("{}-{}", &element.text, &element.fontsize);

                    let text = cache.texts
                        .entry(uid.clone())
                        .or_insert_with(|| Text::new(&element.text, cache.fonts[&element.fontsize].clone()));

                    engine.buttons.insert(element.id.clone(), Button2D::new(ctx, pos_offset, None, Option::from(text), "button_clicked".to_string(), [element.id.to_string()].into()));
                }
                "Image" if !element.hidden && !element.sprite.is_empty() => {
                    let texture = &cache.textures[&element.sprite];
                    engine.buttons.insert(element.id.clone(), Button2D::new(ctx, pos_offset, Option::from(texture), None, "image_clicked".to_string(), [element.id.to_string()].into()));
                }
                _ => {}
            }
        }
    }

    pub fn update(ctx: &mut Context, state: &mut GameState) -> tetra::Result
    {
        Ok(())
    }

    pub fn goto_menu(menu: &str, engine: &mut EngineData, event_manager: & mut EventManager)//, mut scheduler: &Scheduler)
    {
        engine.logger.log("Menu Manager",format!("Going to: {}", menu).as_str());
        // TODO: Switch audio tracks
        engine.menumgr.curmenu = menu.to_string();
        event_manager.kill_all_listeners();
        event_manager.run_script(&engine.game.menus[menu].code);
        event_manager.trigger_event("on_menu_start", &*Vec::new(),engine)

    }
}


