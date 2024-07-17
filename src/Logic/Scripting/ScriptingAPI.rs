pub struct ScriptingAPI {
    actions: HashMap<String, CodeBlockFunction>,
}

impl ScriptingAPI {
    pub fn init_api() -> Result<ScriptingAPI, Box<dyn std::error::Error>> {
        let mut actions = HashMap::new();
        actions.insert(String::from("goto_menu"), Self::goto_menu as CodeBlockFunction);
        actions.insert(String::from("start_night"), Self::start_night as CodeBlockFunction);

        Ok(ScriptingAPI { actions })
    }

    fn goto_menu(engine_data: &mut EngineData, event_manager: & mut EventManager, args: &[Value]) -> bool {
        MenuManager::goto_menu(args[0].as_str().unwrap(), engine_data, event_manager);
        true
    }

    fn start_night(engine_data: &mut EngineData, event_manager: & mut EventManager, args: &[Value]) -> bool {
        // TODO: implement curnight for animatronics
        OfficeManager::init_office("office".to_string(), engine_data, event_manager);
        true
    }
}
