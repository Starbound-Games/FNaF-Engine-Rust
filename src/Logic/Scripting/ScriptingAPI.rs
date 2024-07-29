use std::process;
use tetra::input::is_mouse_button_up;
use tetra::window::quit;

pub struct ScriptingAPI {
    actions: HashMap<String, CodeBlockFunction>,
}

impl ScriptingAPI {
    pub fn init_api() -> Result<ScriptingAPI, Box<dyn std::error::Error>> {
        let mut actions = HashMap::new();
        // Menus
        actions.insert(String::from("start_night"), Self::start_night as CodeBlockFunction);
        actions.insert(String::from("set_background"), Self::set_background as CodeBlockFunction);
        actions.insert(String::from("hide_element"), Self::hide_element as CodeBlockFunction);
        actions.insert(String::from("show_element"), Self::show_element as CodeBlockFunction);
        actions.insert(String::from("setoff"), Self::setoff as CodeBlockFunction);
        actions.insert(String::from("set_text"), Self::set_text as CodeBlockFunction);
        actions.insert(String::from("set_sprite"), Self::set_sprite as CodeBlockFunction);
        actions.insert(String::from("is_button_selected"), Self::is_btn_selected as CodeBlockFunction);
        actions.insert(String::from("is_image_selected"), Self::is_btn_selected as CodeBlockFunction);
        actions.insert(String::from("ebutton_arrows"), Self::enable_arrows as CodeBlockFunction);
        actions.insert(String::from("dbutton_arrows"), Self::disable_arrows as CodeBlockFunction);
        actions.insert(String::from("windowed"), Self::windowed as CodeBlockFunction);
        actions.insert(String::from("fullscreen"), Self::fullscreen as CodeBlockFunction);
        actions.insert(String::from("hide_mouse"), Self::hide_mouse as CodeBlockFunction);
        actions.insert(String::from("show_mouse"), Self::show_mouse as CodeBlockFunction);

        // Office
        actions.insert(String::from("office"), Self::office as CodeBlockFunction);
        actions.insert(String::from("hide_office_object"), Self::hide_office_object as CodeBlockFunction);
        actions.insert(String::from("show_office_object"), Self::show_office_object as CodeBlockFunction);
        actions.insert(String::from("is_mouse_over_object"), Self::is_mouse_over_object as CodeBlockFunction);
        actions.insert(String::from("is_mouse_over_sprite"), Self::is_mouse_over_object as CodeBlockFunction); // Fix when i seperate sprites from objects

        // Shared
        actions.insert(String::from("goto_menu"), Self::goto_menu as CodeBlockFunction);
        actions.insert(String::from("compare_values"), Self::compare_values as CodeBlockFunction);
        actions.insert(String::from("play_sound"), Self::play_audio as CodeBlockFunction);
        actions.insert(String::from("stop_channel"), Self::stop_channel as CodeBlockFunction);
        actions.insert(String::from("set_var"), Self::set_var as CodeBlockFunction);
        actions.insert(String::from("set_data"), Self::set_datavalue as CodeBlockFunction);
        actions.insert(String::from("quit"), Self::quit as CodeBlockFunction);
        actions.insert(String::from("line"), Self::line as CodeBlockFunction);
        actions.insert(String::from("comment"), Self::comment as CodeBlockFunction);


        Ok(ScriptingAPI { actions })
    }

    fn comment(engine_data: &mut EngineData, event_manager: & mut EventManager, args: &[Value]) -> bool {
        true
    }

    fn line(engine_data: &mut EngineData, event_manager: & mut EventManager, args: &[Value]) -> bool {
        engine_data.logger.log("ScriptingAPI", format!("DEBUG: {}", args[0].as_str().unwrap()).as_str());
        true
    }

    fn quit(engine_data: &mut EngineData, event_manager: & mut EventManager, args: &[Value]) -> bool {
        process::exit(0);
    }

    fn is_mouse_over_object(engine_data: &mut EngineData, event_manager: &mut EventManager, args: &[Value]) -> bool {
        let sprite_name = match args.get(0) {
            Some(value) => match value.as_str() {
                Some(name) => name,
                None => {
                    engine_data.logger.log_error("ScriptingAPI", "Invalid argument type for sprite name.");
                    return false;
                }
            },
            None => {
                engine_data.logger.log_error("ScriptingAPI", "No argument provided for sprite name.");
                return false;
            }
        };
        match engine_data.officemgr.game_data.office.objects.get(sprite_name) {
            Some(sprite) => sprite.hovered,
            None => {
                engine_data.logger.log_error("ScriptingAPI", "Object not found.");
                false
            }
        }
    }

    fn hide_office_object(engine_data: &mut EngineData, event_manager: & mut EventManager, args: &[Value]) -> bool {
        let sprite_name = match args.get(0) {
            Some(value) => match value.as_str() {
                Some(name) => name,
                None => {
                    engine_data.logger.log_error("ScriptingAPI", "Invalid argument type for sprite name.");
                    return false;
                }
            },
            None => {
                engine_data.logger.log_error("ScriptingAPI", "No argument provided for sprite name.");
                return false;
            }
        };
        println!("{}", sprite_name);
        match engine_data.officemgr.game_data.office.objects.get_mut(sprite_name) {
            Some(sprite) => {sprite.visible = false; true},
            None => {
             //   engine_data.logger.log_error("ScriptingAPI", "Object not found.");
                false
            }
        }
    }

    fn show_office_object(engine_data: &mut EngineData, event_manager: & mut EventManager, args: &[Value]) -> bool {
        let sprite_name = match args.get(0) {
            Some(value) => match value.as_str() {
                Some(name) => name,
                None => {
                    engine_data.logger.log_error("ScriptingAPI", "Invalid argument type for sprite name.");
                    return false;
                }
            },
            None => {
                engine_data.logger.log_error("ScriptingAPI", "No argument provided for sprite name.");
                return false;
            }
        };
        println!("{}", sprite_name);
        match engine_data.officemgr.game_data.office.objects.get_mut(sprite_name) {
            Some(sprite) => {sprite.visible = true; true},
            None => {
             //   engine_data.logger.log_error("ScriptingAPI", "Object not found.");
                false
            }
        }
    }


    fn hide_mouse(engine_data: &mut EngineData, event_manager: & mut EventManager, args: &[Value]) -> bool {
        engine_data.show_mouse = false;
        engine_data.update_context = true;
        true
    }

    fn show_mouse(engine_data: &mut EngineData, event_manager: & mut EventManager, args: &[Value]) -> bool {
        engine_data.show_mouse = true;
        engine_data.update_context = true;
        true
    }

    fn windowed(engine_data: &mut EngineData, event_manager: & mut EventManager, args: &[Value]) -> bool {
        engine_data.fullscreen = false;
        engine_data.update_context = true;
        true
    }

    fn fullscreen(engine_data: &mut EngineData, event_manager: & mut EventManager, args: &[Value]) -> bool {
        engine_data.fullscreen = true;
        engine_data.update_context = true;
        true
    }

    fn stop_channel(engine_data: &mut EngineData, event_manager: & mut EventManager, args: &[Value]) -> bool {
        engine_data.audio.stop_channel(args[0].as_str().expect("Error while casting str to int").parse().unwrap());
        true
    }

    fn play_audio(engine_data: &mut EngineData, event_manager: & mut EventManager, args: &[Value]) -> bool {
        if (args[1].as_str().unwrap().is_empty()) {
            engine_data.audio.play(args[0].as_str().unwrap(), args[2].as_bool().unwrap());
        }
        else {
            engine_data.audio.play_on_channel(args[0].as_str().unwrap(), args[2].as_bool().unwrap(), args[1].as_str().expect("Error while casting str").parse().unwrap());
        }
        true
    }

    fn goto_menu(engine_data: &mut EngineData, event_manager: & mut EventManager, args: &[Value]) -> bool {
        let binding = event_manager.get_expr(args[0].as_str().unwrap(), engine_data);
        let menu = binding.as_str();
        MenuManager::goto_menu(menu, engine_data, event_manager);
        true
    }

    fn is_btn_selected(engine_data: &mut EngineData, event_manager: & mut EventManager, args: &[Value]) -> bool {
        let button_name = event_manager.get_expr(args[0].as_str().unwrap(), engine_data);
        return MenuManager::is_button_selected(button_name, engine_data);
    }

    fn disable_arrows(engine_data: &mut EngineData, event_manager: & mut EventManager, args: &[Value]) -> bool {
        engine_data.menumgr.arrows_enabled = false;
        true
    }

    fn enable_arrows(engine_data: &mut EngineData, event_manager: & mut EventManager, args: &[Value]) -> bool {
        engine_data.menumgr.arrows_enabled = true;
        true
    }

    fn set_sprite(engine_data: &mut EngineData, event_manager: & mut EventManager, args: &[Value]) -> bool {
        let element = event_manager.get_expr(args[0].as_str().unwrap(), engine_data);
        let value = event_manager.get_expr(args[1].as_str().unwrap(), engine_data);
        MenuManager::set_element_sprite(element.as_str(), value, engine_data);
        true
    }

    fn set_text(engine_data: &mut EngineData, event_manager: & mut EventManager, args: &[Value]) -> bool {
        let element = event_manager.get_expr(args[0].as_str().unwrap(), engine_data);
        let value = event_manager.get_expr(args[1].as_str().unwrap(), engine_data);
        MenuManager::set_element_text(element.as_str(), value, engine_data);
        true
    }

    fn set_var(engine_data: &mut EngineData, event_manager: & mut EventManager, args: &[Value]) -> bool {
        let name = event_manager.get_expr(args[0].as_str().unwrap(), engine_data);
        let value = event_manager.get_expr(args[1].as_str().unwrap(), engine_data);
        event_manager.set_variable_value(name, value);
        true
    }

    fn set_datavalue(engine_data: &mut EngineData, event_manager: & mut EventManager, args: &[Value]) -> bool {
        let name = event_manager.get_expr(args[0].as_str().unwrap(), engine_data);
        let value = event_manager.get_expr(args[1].as_str().unwrap(), engine_data);
        event_manager.set_data_value(name, value);
        true
    }

    fn setoff(engine_data: &mut EngineData, event_manager: & mut EventManager, args: &[Value]) -> bool {
        let office = event_manager.get_expr(args[0].as_str().unwrap(), engine_data);
        engine_data.officemgr.curoffice = office;
        true
    }

    fn office(engine_data: &mut EngineData, event_manager: & mut EventManager, args: &[Value]) -> bool {
        let office = event_manager.get_expr(args[0].as_str().unwrap(), engine_data);
        OfficeManager::init_office(office, engine_data.officemgr.game_data.night.clone(), engine_data, event_manager);
        true
    }

    fn start_night(engine_data: &mut EngineData, event_manager: & mut EventManager, args: &[Value]) -> bool {
        let night = event_manager.get_expr(args[0].as_str().unwrap(), engine_data);
        OfficeManager::init_office(engine_data.officemgr.curoffice.clone(), night.parse().unwrap(), engine_data, event_manager);
        true
    }

    fn set_background(engine_data: &mut EngineData, event_manager: & mut EventManager, args: &[Value]) -> bool {
        MenuManager::set_background(args[0].as_str().unwrap(), engine_data);
        true
    }

    fn show_element(engine_data: &mut EngineData, event_manager: & mut EventManager, args: &[Value]) -> bool {
        let binding = event_manager.get_expr(args[0].as_str().unwrap(), engine_data);
        let element = binding.as_str();
        MenuManager::show_element(element, engine_data);
        true
    }

    fn hide_element(engine_data: &mut EngineData, event_manager: & mut EventManager, args: &[Value]) -> bool {
        let binding = event_manager.get_expr(args[0].as_str().unwrap(), engine_data);
        let element = binding.as_str();
        MenuManager::hide_element(element, engine_data);
        true
    }

    fn compare_values(engine_data: &mut EngineData, event_manager: & mut EventManager, args: &[Value]) -> bool {
        let lhs = event_manager.get_expr(args[0].as_str().unwrap(), engine_data);
        let rhs = event_manager.get_expr(args[2].as_str().unwrap(), engine_data);

        match args[1].as_str().unwrap() {
            "==" => lhs == rhs,
            "<>" => lhs != rhs,
            ">"  => lhs > rhs,
            "<"  => lhs < rhs,
            ">=" => lhs >= rhs,
            "<=" => lhs <= rhs,
            _    => panic!("Invalid operator"),
        }
    }

    fn TickEvents(engine_data: &mut EngineData, event_manager: &mut EventManager, ctx: &mut Context) -> Result<(), TetraError> {
        event_manager.trigger_event("on_game_loop", &[String::new()], engine_data);
        let tick = engine_data.clock.current_tick.lock().unwrap().clone();
        event_manager.trigger_event(
            "current_tick_equals",
            &[tick.to_string()],
            engine_data,
        );

        let mut event_name_clicked: String = String::new();
        let mut event_args_clicked: Vec<String> = Vec::new();
        let mut selected_button_id: String = String::new();
        let mut is_image: bool = false;;

        for (k, mut button) in &mut engine_data.buttons {
            button.update(ctx, engine_data.officemgr.scroll_x);
            if button.is_hovered {
                match (engine_data.scene) {
                    0 => {
                        engine_data.menumgr.selected_button_id = k.clone()
                    }
                    1 => {
                        if (!engine_data.officemgr.hovered_id.is_empty()) {
                            engine_data.officemgr.game_data.office.objects.get_mut(engine_data.officemgr.hovered_id.as_str()).expect("Previous Object not found??").hovered = false;
                        }
                        engine_data.officemgr.hovered_id = k.clone();
                        engine_data.officemgr.game_data.office.objects.get_mut(k.clone().as_str()).expect("Object not found.").hovered = true;
                    }
                    _ => {}
                }
                if button.is_clicked {
                    event_name_clicked = button.event.clone();
                    event_args_clicked = button.event_args.clone();
                }
                selected_button_id = k.clone();
                is_image = button.is_image.clone();
            }
        }

        if (!selected_button_id.is_empty()) {
            if (!engine_data.hovered) {
                event_manager
                    .trigger_event(if is_image { "image_selected" } else { "button_selected" }, &[selected_button_id], engine_data);
                event_manager
                    .trigger_event("any_button_selected", &[], engine_data);
                engine_data.hovered = true;
            }

            if (!engine_data.disabled_clicks[3]) {
                if (!event_name_clicked.is_empty()) {
                    event_manager
                        .trigger_event(event_name_clicked.as_str(), &*event_args_clicked, engine_data);
                    engine_data.disabled_clicks[3] = true;
                }
            } else {
                engine_data.disabled_clicks[3] = false
            }
        }
        else {
            engine_data.hovered = false;
            engine_data.menumgr.selected_button_id = String::new();
        }

        let mouse_buttons = [
            (MouseButton::Left, "user_left_clicked"),
            (MouseButton::Right, "user_right_clicked"),
            (MouseButton::Middle, "user_middle_clicked"),
        ];

        for (button, event_name) in &mouse_buttons {
            if !engine_data.disabled_clicks[MouseButton_to_i32(*button)] && is_mouse_button_down(ctx, *button) {
                event_manager.trigger_event(event_name, &[], engine_data);
                engine_data.disabled_clicks[MouseButton_to_i32(*button)] = true;
            }
            else if engine_data.disabled_clicks[MouseButton_to_i32(*button)] && is_mouse_button_up(ctx, *button) {
                engine_data.disabled_clicks[MouseButton_to_i32(*button)] = false;
            }
        }
        Ok(())
    }
}

fn MouseButton_to_i32(value: MouseButton) -> usize {
    match value {
        MouseButton::Left => 0,
        MouseButton::Middle => 1,
        MouseButton::Right => 2,
        _ => unimplemented!()
    }
}