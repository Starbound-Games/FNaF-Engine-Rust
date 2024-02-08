struct AssetLoader {
    pub textures: HashMap<String, Texture>,
    pub fonts: HashMap<i32, Font>,
}

impl AssetLoader {
    fn load_assets(
        ctx: &mut Context,
        game: &Game,
        menumgr: &MenuManager,
        assets: &PathBuf,
        logger: &Logger,
    ) -> tetra::Result<AssetLoader> {

        let mut textures: HashMap<String, Texture> = HashMap::new();
        let mut fonts: HashMap<i32, Font> = HashMap::new();
        println!("{:?}", assets.to_str());


        // Init Engine
        if !game.menus[&menumgr.curmenu].properties.BackgroundImage.is_empty() {
            let mut path = assets
                .join("sprites")
                .join(&game.menus[&menumgr.curmenu].properties.BackgroundImage.replace("\\", "/"))
                .to_str()
                .unwrap()
                .to_string();

            if Path::new(&path).exists() {
                textures.insert(
                    String::from(&game.menus[&menumgr.curmenu].properties.BackgroundImage),
                    Texture::new(ctx, path)?,
                );
            }
        }

        for element in &game.menus[&menumgr.curmenu].elements {
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

        logger.log("Main", "Loaded Assets");
        Ok(AssetLoader {
            textures: textures,
            fonts: fonts,
        })
    }
}