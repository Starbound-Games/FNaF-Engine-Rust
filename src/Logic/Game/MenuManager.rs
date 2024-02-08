include!("../Scripting/Scheduler.rs");

pub struct MenuManager
{
    curmenu: String,
}

impl MenuManager {
    pub fn new() -> MenuManager {
        let curmenu = String::new();
        MenuManager { curmenu }
    }

    pub fn update(ctx: &mut Context, state: &mut GameState) -> tetra::Result
    {
        Ok(())
    }

    pub fn goto_menu(menu: &str, state: &mut GameState)
    {
        state.menumgr.curmenu = menu.to_string();
    }
}


