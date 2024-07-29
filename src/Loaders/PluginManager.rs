use libloading::{Library, Symbol};

pub trait Plugin {
    fn initialize(&mut self);
    fn update(&mut self, ctx: &mut Context);
    fn shutdown(&mut self);
}

pub trait PluginFactory {
    fn create_plugin(&self) -> Box<dyn Plugin>;
}
pub struct PluginManager {
    plugins: Vec<Box<dyn Plugin>>,
    libraries: Vec<Library>, // Hold onto the libraries to ensure they aren't dropped
}

impl PluginManager {
    pub fn new() -> Self {
        PluginManager {
            plugins: Vec::new(),
            libraries: Vec::new(),
        }
    }

    pub unsafe fn load_plugin(&mut self, path: &str) {
        let lib = Library::new(PathBuf::from(path)).unwrap();
        self.libraries.push(lib);

        let factory: Symbol<extern "C" fn() -> Box<dyn PluginFactory>> = {
            self.libraries.last().unwrap().get(b"create_plugin").unwrap()
        };

        let plugin_factory = factory();
        let plugin = plugin_factory.create_plugin();
        self.plugins.push(plugin);
    }

    pub fn initialize_plugins(&mut self) {
        for plugin in self.plugins.iter_mut() {
            plugin.initialize();
        }
    }

    pub fn update_plugins(&mut self, ctx: &mut Context) {
        for plugin in self.plugins.iter_mut() {
            plugin.update(ctx);
        }
    }

    pub fn shutdown_plugins(&mut self) {
        for plugin in self.plugins.iter_mut() {
            plugin.shutdown();
        }
    }
}
