use std::fmt::format;

struct AssetLoader {
    pub textures: HashMap<String, Texture>,
    pub fonts: HashMap<i32, Font>,
}

impl AssetLoader {
    fn load_menus(
        ctx: &mut Context,
        game: &Game,
        menumgr: &MenuManager,
        assets: &PathBuf,
        logger: &Logger,
    ) -> tetra::Result<AssetLoader> {

        let mut textures: HashMap<String, Texture> = HashMap::new();
        let mut fonts: HashMap<i32, Font> = HashMap::new();

        for (k, v) in &game.menus {
            logger.log("AssetLoader", k);
        }

        // Init Engine
        for menu in game.menus.values() {
            if !&menu.properties.BackgroundImage.is_empty() {
                let mut path = assets
                    .join("sprites")
                    .join(&menu.properties.BackgroundImage.replace("\\", "/"))
                    .to_str()
                    .unwrap()
                    .to_string();

                if Path::new(&path).exists() {
                    textures.insert(
                        String::from(&menu.properties.BackgroundImage),
                        Texture::new(ctx, path)?,
                    );
                }
            }


            for element in &menu.elements {
                if !element.sprite.is_empty() {
                    let mut path = assets
                        .join("sprites")
                        .join(&element.sprite.replace("\\", "/"))
                        .to_str()
                        .unwrap()
                        .to_string();

                    if Path::new(&path).exists() {
                        textures.insert(String::from(&element.sprite), Texture::new(ctx, path)?);
                    }
                }

                if !fonts.contains_key(&element.fontsize) {
                    fonts.insert(
                        element.fontsize,
                        Font::vector(ctx, "./src/Arial.ttf", element.fontsize as f32)?,
                    );
                }
            }
        }

        logger.log("AssetLoader", "Loaded Assets");
        Ok(AssetLoader {
            textures: textures,
            fonts: fonts,
        })
    }

    fn load_offices(
        ctx: &mut Context,
        game: &Game,
        assets: &PathBuf,
        logger: &Logger,
    ) -> tetra::Result<AssetLoader> {

        let mut textures: HashMap<String, Texture> = HashMap::new();
        let mut fonts: HashMap<i32, Font> = HashMap::new();

        // Init Engine
        for office in game.offices.values() {
            for state in &mut office.states.values() {
                if !state.is_empty() {
                    let mut path = assets
                        .join("sprites")
                        .join(&state.replace("\\", "/"))
                        .to_str()
                        .unwrap()
                        .to_string();

                    if Path::new(&path).exists() {
                        textures.insert(String::from(state), Texture::new(ctx, path)?);
                    }
                }
            }

            for obj in &office.objects {
                if !obj.sprite.is_empty() {
                    let mut path = assets
                        .join("sprites")
                        .join(&obj.sprite.replace("\\", "/"))
                        .to_str()
                        .unwrap()
                        .to_string();

                    if Path::new(&path).exists() {
                        textures.insert(String::from(&obj.sprite), Texture::new(ctx, path)?);
                    }
                }
                if !obj.on_sprite.is_empty() {
                    let mut path = assets
                        .join("sprites")
                        .join(&obj.sprite.replace("\\", "/"))
                        .to_str()
                        .unwrap()
                        .to_string();

                    if Path::new(&path).exists() {
                        textures.insert(String::from(&obj.sprite), Texture::new(ctx, path)?);
                    }
                }
            }
        }

        logger.log("AssetLoader", "Loaded Assets");
        Ok(AssetLoader {
            textures: textures,
            fonts: fonts,
        })
    }
}