use mlua::{Lua, Function, UserData, UserDataMethods, FromLua, FromLuaMulti, IntoLuaMulti};

struct LuaBindings {
    lua: Lua,
}

impl LuaBindings {
    fn new() -> Self {
        LuaBindings { lua: Lua::new() }
    }

    fn exec_script(&self, script: &str) -> mlua::Result<()> {
        self.lua.load(script).exec()
    }

    fn set_global<T: 'static + mlua::UserData>(&self, name: &str, value: T)  -> mlua::Result<()>
    where
        T: IntoLuaMulti<'static>,
    {
        self.lua.globals().set(name, value)
    }

    fn call_function<A, R>(&self, func_name: &str, args: A)
    where
        A: IntoLuaMulti<'static>,
        R: FromLuaMulti<'static>,
    {
    //    let func: Function = self.lua.globals().get(func_name).expect("UNABLE TO FIND FUNCTION");
       // func.call(args).expect("FAILED TO RUN FUNCTION");
    }
}



impl UserData for GameData {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("Night", |_, this| Ok(this.night));
        fields.add_field_method_get("Time", |_, this| Ok(this.time));
      //  fields.add_field_method_get("Office", |_, this| Ok(this.office));
       // fields.add_field_method_get("Settings", |_, this| Ok(&this.settings));
     //   fields.add_field_method_get("Player", |_, this| Ok(&this.player));
        //fields.add_field_method_get("Animations", |_, this| Ok(&this.office.animations));
       // fields.add_field_method_get("Cameras", |_, this| Ok(&this.cameras));
      //  fields.add_field_method_get("CameraUI", |_, this| Ok(&this.cam_ui));

        // TODO: Mouse
    }

    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
    //    methods.add_method("SetTime", |_, &mut this, val| Ok(this.set_time(val)));
    //    methods.add_method("Quit", |_, &mut this, ()| Ok(this.quit()));
      //  methods.add_method("DisableTime", |_, &mut this, ()| Ok(this.disable_time()));
      //  methods.add_method("EnableTime", |_, &mut this, ()| Ok(this.enable_time()));
        methods.add_method("GotoMenu", |_, mut this, ()| Ok(())); // TODO
        methods.add_method("IsKeyDown", |_, mut this, ()| Ok(())); // TODO

    }
}

impl UserData for OfficeData {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
      //  fields.add_field_method_get("States", |_, this| Ok(&this.states));
    //    fields.add_field_method_set("States", |_, mut this, val| Ok(this.states = val));
        fields.add_field_method_get("State", |_, this| Ok(this.state.clone()));
    }

    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
      //  methods.add_method("SetState", |_, mut this, val| Ok(this.set_state(val)));
    }
}

impl UserData for Settings {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("Mask", |_, this| Ok(this.mask));
        fields.add_field_method_set("Mask", |_, mut this, val| Ok(this.mask = val));
        fields.add_field_method_get("Flashlight", |_, this| Ok(this.flashlight));
        fields.add_field_method_set("Flashlight", |_, mut this, val| Ok(this.flashlight = val));
        fields.add_field_method_get("Toxic", |_, this| Ok(this.toxic));
        fields.add_field_method_set("Toxic", |_, mut this, val| Ok(this.toxic = val));
        fields.add_field_method_get("Panorama", |_, this| Ok(this.panorama));
        fields.add_field_method_set("Panorama", |_, mut this, val| Ok(this.panorama = val));
        fields.add_field_method_get("Power", |_, this| Ok(this.power));
        fields.add_field_method_set("Power", |_, mut this, val| Ok(this.power = val));
    }

    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
    }
}

impl UserData for Player {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("IsCameraUp", |_, this| Ok(this.is_camera_up));
        fields.add_field_method_set("IsCameraUp", |_, mut this, val| Ok(this.is_camera_up = val));
        fields.add_field_method_get("CameraButtonToggle", |_, this| Ok(this.camera_button_toggle));
        fields.add_field_method_set("CameraButtonToggle", |_, mut this, val| Ok(this.camera_button_toggle = val));
        fields.add_field_method_get("IsMaskOn", |_, this| Ok(this.is_mask_on));
        fields.add_field_method_set("IsMaskOn", |_, mut this, val| Ok(this.is_mask_on = val));
        fields.add_field_method_get("IsFlashlightOn", |_, this| Ok(this.is_flashlight_on));
        fields.add_field_method_set("IsFlashlightOn", |_, mut this, val| Ok(this.is_flashlight_on = val));
        fields.add_field_method_get("CurrentCamera", |_, this| Ok(this.current_camera.clone()));
        fields.add_field_method_set("CurrentCamera", |_, mut this, val| Ok(this.current_camera = val));
    }

    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
    //    methods.add_method("SetCamera", |_, mut this, val| Ok(this.set_camera(val)));
    }
}

// TODO: ANIMTION DATA STRUCTURES
impl UserData for Animation { // TODO
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
   //     fields.add_field_method_get("ID", |_, this| Ok(&this.id));
//        fields.add_field_method_set("ID", |_, this, val| Ok(this.id = val)); // ID should be readonly
   //     fields.add_field_method_get("IsPlaying", |_, this| Ok(&this.is_playing));
  //      fields.add_field_method_set("IsPlaying", |_, this, val| Ok(this.is_playing = val));
 //       fields.add_field_method_get("IsReversed", |_, this| Ok(&this.is_reversed));
    //    fields.add_field_method_set("IsReversed", |_, this, val| Ok(this.is_reversed = val));
    }

    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
    }
}

impl UserData for Camera {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("Panorama", |_, this| Ok(this.panorama));
        fields.add_field_method_set("Panorama", |_, this, val| Ok(this.panorama = val));
        fields.add_field_method_get("State", |_, this| Ok(this.state.clone()));
        fields.add_field_method_set("State", |_, this, val| Ok(this.state = val));
        fields.add_field_method_get("States", |_, this| Ok(this.states.clone()));
        fields.add_field_method_set("States", |_, this, val| Ok(this.states = val));
        fields.add_field_method_get("StaticCamera", |_, this| Ok(this.static_camera));
        fields.add_field_method_set("StaticCamera", |_, this, val| Ok(this.static_camera = val));
        fields.add_field_method_get("Scroll", |_, this| Ok(this.scroll));
        fields.add_field_method_set("Scroll", |_, this, val| Ok(this.scroll = val));
    }

    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
       // methods.add_method("SetState", |_, mut this, val| Ok(this.set_state(val)));
    }
}

impl UserData for &CamUI { // TODO
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
     //   fields.add_field_method_get("Buttons", |_, this| Ok(this.buttons.clone()));
      //  fields.add_field_method_set("Buttons", |_, this, val| Ok(this.buttons = val));
        fields.add_field_method_get("MusicBox", |_, this| Ok(this.music_box.clone()));
   //     fields.add_field_method_set("MusicBox", |_, this, val| Ok(this.music_box = val));
      //  fields.add_field_method_get("Sprites", |_, this| Ok(this.sprites.clone()));
   //     fields.add_field_method_set("Sprites", |_, this, val| Ok(this.sprites = val));
    }

    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
    }
}

impl UserData for Animatronic {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("State", |_, this| Ok(this.state.clone()));
        fields.add_field_method_set("State", |_, this, val| Ok(this.state = val));
        fields.add_field_method_get("Name", |_, this| Ok(this.name.clone()));
        fields.add_field_method_set("Name", |_, this, val| Ok(this.name = val));
      //  fields.add_field_method_get("Path", |_, this| Ok(this.path));
      //  fields.add_field_method_set("Path", |_, this, val| Ok(this.path = val));
       // fields.add_field_method_get("AI", |_, this| Ok(this.ai));
        fields.add_field_method_set("AI", |_, this, val| Ok(this.ai = val));
        fields.add_field_method_get("IgnoresMask", |_, this| Ok(this.ignores_mask));
        fields.add_field_method_set("IgnoresMask", |_, this, val| Ok(this.ignores_mask = val));
       // fields.add_field_method_get("Jumpscare", |_, this| Ok(this.jumpscare));
      //  fields.add_field_method_set("Jumpscare", |_, this, val| Ok(this.jumpscare = val));
       // fields.add_field_method_get("Location", |_, this| Ok(this.location.clone()));
    //    fields.add_field_method_set("Location", |_, this, val| Ok(this.location = val));
        fields.add_field_method_get("LocationIndex", |_, this| Ok(this.location_index));
        fields.add_field_method_set("LocationIndex", |_, this, val| Ok(this.location_index = val));
        fields.add_field_method_get("MoveTime", |_, this| Ok(this.move_time));
        fields.add_field_method_set("MoveTime", |_, this, val| Ok(this.move_time = val));
        fields.add_field_method_get("Phantom", |_, this| Ok(this.phantom));
        fields.add_field_method_set("Phantom", |_, this, val| Ok(this.phantom = val));
    }

    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
    }
}

impl UserData for PathNode {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
   //     fields.add_field_method_get("Path", |_, this| Ok(this.path.clone()));
        //fields.add_field_method_set("Path", |_, this, val| Ok(this.path = val));
        fields.add_field_method_get("State", |_, this| Ok(this.state.clone()));
        fields.add_field_method_set("State", |_, this, val| Ok(this.state = val));
        fields.add_field_method_get("ID", |_, this| Ok(this.id.clone()));
        fields.add_field_method_set("ID", |_, this, val| Ok(this.id = val));
        fields.add_field_method_get("CamID", |_, this| Ok(this.camid.clone()));
        fields.add_field_method_set("CamID", |_, this, val| Ok(this.camid = val));
        fields.add_field_method_get("Chance", |_, this| Ok(this.chance));
        fields.add_field_method_set("Chance", |_, this, val| Ok(this.chance = val));
        fields.add_field_method_get("Type", |_, this| Ok(this.type_.clone()));
        fields.add_field_method_set("Type", |_, this, val| Ok(this.type_ = val));
    }

    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
    }
}

impl UserData for Power {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("Level", |_, this| Ok(this.level));
        fields.add_field_method_set("Level", |_, this, val| Ok(this.level = val));
        fields.add_field_method_get("Usage", |_, this| Ok(this.usage));
        fields.add_field_method_set("Usage", |_, this, val| Ok(this.usage = val));
        fields.add_field_method_get("Enabled", |_, this| Ok(this.enabled));
        fields.add_field_method_set("Enabled", |_, this, val| Ok(this.enabled = val));
        fields.add_field_method_get("UCN", |_, this| Ok(this.ucn));
        fields.add_field_method_set("UCN", |_, this, val| Ok(this.ucn = val));
        fields.add_field_method_get("PowerOutAnimation", |_, this| Ok(this.power_out_animation.clone()));
        fields.add_field_method_set("PowerOutAnimation", |_, this, val| Ok(this.power_out_animation = val));
        fields.add_field_method_get("AnimatronicJumpscare", |_, this| Ok(this.animatronic_jumpscare.clone()));
        fields.add_field_method_set("AnimatronicJumpscare", |_, this, val| Ok(this.animatronic_jumpscare = val));
    }

    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
    }
}

