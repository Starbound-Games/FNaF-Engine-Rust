struct AssetLoader {
    pub textures: HashMap<String, Texture>,
    pub fonts: HashMap<i32, Font>,
    pub game: Game,
}

impl AssetLoader {
    fn load_assets (
        ctx: &mut Context,
        curmenu: &str,
        logger: &Logger,
    ) -> tetra::Result<AssetLoader> {
        let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("target/debug/assets").unwrap();
        let mut textures: HashMap<String, Texture> = HashMap::new();
        let mut fonts: HashMap<i32, Font> = HashMap::new();
        println!("{:?}", assets.to_str());

        // Load Game
        let game = GameLoader::Load(assets.join("game.json").to_str().unwrap());

        for (k, v) in &game.menus {
            logger.log("Main", k);
        }

        // Init Engine
        if !game.menus[curmenu].properties.BackgroundImage.is_empty() {
            let mut path = assets
                .join("sprites")
                .join(&game.menus[curmenu].properties.BackgroundImage.replace("\\", "/"))
                .to_str()
                .unwrap()
                .to_string();

            if Path::new(&path).exists() {
                textures.insert(
                    String::from(&game.menus[curmenu].properties.BackgroundImage),
                    Texture::new(ctx, path)?,
                );
            }
        }

        for element in &game.menus[curmenu].elements {
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
            game: game,
        })
    }
}