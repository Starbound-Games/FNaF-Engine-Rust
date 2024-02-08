

pub struct ScriptingAPI {
    actions: HashMap<String, Box<dyn Fn() -> bool + Send>>,
}

impl ScriptingAPI {
    pub fn init_api() -> Result<ScriptingAPI, Box<dyn std::error::Error>> {
        let mut actions = HashMap::new();
        actions.insert(String::from("goto_menu"), Box::new(Self::goto_menu) as Box<dyn Fn() -> bool + Send>);
        Ok(ScriptingAPI { actions })
    }

    fn goto_menu() -> bool {
        true
    }
}
