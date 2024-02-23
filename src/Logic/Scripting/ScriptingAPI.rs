

pub struct ScriptingAPI {
    actions: HashMap<String, Box<dyn Fn(Vec<Value>, &GameState) -> bool + Send>>,
}

impl ScriptingAPI {
    pub fn init_api(game_state: &GameState) -> Result<ScriptingAPI, Box<dyn std::error::Error>> {
        let mut actions = HashMap::new();
        actions.insert(String::from("goto_menu"), Box::new(Self::goto_menu) as Box<dyn Fn(Vec<Value>, &GameState) -> bool + Send>);
        Ok(ScriptingAPI { actions })
    }

    fn goto_menu(args: Vec<Value>, mut State: &GameState) -> bool
    {
        State.menumgr.curmenu = args[0].to_string();
        return true;
    }
}