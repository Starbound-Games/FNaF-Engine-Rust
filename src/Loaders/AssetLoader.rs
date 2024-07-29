use image::{EncodableLayout, ImageBuffer, ImageFormat, Rgba, RgbaImage};
use image::DynamicImage::ImageRgb8;
use tetra::graphics::{DrawParams, ImageData, TextureFormat};

pub struct AssetLoader {
    pub animations: HashMap<String, RevAnimation>,
    pub textures: HashMap<String, Texture>,
    pub fonts: HashMap<i32, Font>,
}

impl AssetLoader {
    fn load_menus(
        ctx: &mut Context,
        game: &Game,
        assets: &PathBuf,
        logger: &Logger,
    ) -> tetra::Result<AssetLoader> {
        let mut textures: HashMap<String, Texture> = HashMap::new();
        let mut fonts: HashMap<i32, Font> = HashMap::new();

        for (k, v) in &game.menus {
            logger.log("AssetLoader", k);
        }

        for menu in game.menus.values() {
            if !menu.properties.BackgroundImage.is_empty() {
                let path = assets
                    .join("sprites")
                    .join(&menu.properties.BackgroundImage.replace("\\", "/"));

                if path.exists() {
                    textures.insert(
                        menu.properties.BackgroundImage.clone(),
                        Texture::new(ctx, path.to_str().unwrap())?,
                    );
                }
            }

            for element in &menu.elements {
                if !element.sprite.is_empty() {
                    let path = assets
                        .join("sprites")
                        .join(&element.sprite.replace("\\", "/"));

                    if path.exists() {
                        textures.insert(element.sprite.clone(), Texture::new(ctx, path.to_str().unwrap())?);
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

        logger.log("AssetLoader", "Loaded Menus");
        Ok(AssetLoader {
            animations: HashMap::new(),
            textures,
            fonts,
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

        for office in game.offices.values() {
            for state in office.states.values() {
                if !state.is_empty() {
                    let path = assets
                        .join("sprites")
                        .join(&state.replace("\\", "/"));

                    if path.exists() {
                        textures.insert(state.clone(), Texture::new(ctx, path.to_str().unwrap())?);
                    }
                }
            }

            if !&office.uibuttons.camera.image.is_empty() {
                let path = assets
                    .join("sprites")
                    .join(&office.uibuttons.camera.image.replace("\\", "/"));

                if path.exists() && !textures.contains_key(&office.uibuttons.camera.image) {
                    textures.insert(office.uibuttons.camera.image.clone(), Texture::new(ctx, path.to_str().unwrap())?);
                }
            }

            if !&office.uibuttons.mask.image.is_empty() {
                let path = assets
                    .join("sprites")
                    .join(&office.uibuttons.mask.image.replace("\\", "/"));

                if path.exists() && !textures.contains_key(&office.uibuttons.mask.image) {
                    textures.insert(office.uibuttons.mask.image.clone(), Texture::new(ctx, path.to_str().unwrap())?);
                }
            }

            for obj in &office.objects {
                if !obj.sprite.is_empty() {
                    let path = assets
                        .join("sprites")
                        .join(&obj.sprite.replace("\\", "/"));

                    if path.exists() {
                        textures.insert(obj.sprite.clone(), Texture::new(ctx, path.to_str().unwrap())?);
                    }
                }

                if !obj.on_sprite.is_empty() {
                    let path = assets
                        .join("sprites")
                        .join(&obj.on_sprite.replace("\\", "/"));

                    if path.exists() {
                        textures.insert(obj.on_sprite.clone(), Texture::new(ctx, path.to_str().unwrap())?);
                    }
                }
            }
        }

        logger.log("AssetLoader", "Loaded Offices");
        Ok(AssetLoader {
            animations: HashMap::new(),
            textures,
            fonts,
        })
    }

    fn load_animations(
        ctx: &mut Context,
        assets: &PathBuf,
        logger: &Logger,
        game: &Game
    ) -> tetra::Result<AssetLoader> {
        let animations = Arc::new(Mutex::new(HashMap::new()));

        for (entry, anim_json) in &game.animations {
            let path = PathBuf::from(entry);
            let file_stem = path.file_stem().unwrap().to_str().unwrap().to_string();

            let mut textures = Vec::with_capacity(anim_json.len());
           // let mut frames = Vec::with_capacity(anim_json.len());
            let mut durations = Vec::with_capacity(anim_json.len());

            for json in anim_json {
                let sprite_path = assets.join("sprites").join(&json.sprite.replace("\\", "/"));
                let tex = Texture::new(ctx, &sprite_path).unwrap();
                durations.push(Duration::from_secs_f64(json.duration as f64 * 0.04));
                textures.push(tex);
            }

         //   let max_height = textures[0].height();
         //   let rows = (textures.len() / 10).max(1);

       //     for row in 0..rows {
       //         let y_position = (max_height * row as i32) as f32;
        //        let rects = Rectangle::row(0.0, y_position, textures[0].width() as f32, max_height as f32)
       //             .take(textures.len().min(10));
       //         frames.extend(rects);
       //     }

          //  let total_width: i32 = frames.iter()
          //      .filter(|t| t.y == 0.0)
        //        .map(|t| t.width as i32)
         //       .sum();

       //     let total_height: i32 = frames.iter()
       //         .filter(|t| t.x == 0.0)
       //         .map(|t| t.height as i32)
       //         .sum();

         //   if let Ok(canvas) = graphics::Canvas::new(ctx, total_width, total_height) {
         //       graphics::set_canvas(ctx, &canvas);
         //       graphics::clear(ctx, Color::rgba(0.0, 0.0, 0.0, 0.0));

        //        for (i, frame) in frames.iter().enumerate() {
        //            textures[i].draw(ctx, Vec2::new(frame.x, frame.y));
        //        }

         //       graphics::reset_canvas(ctx);

                let mut animation = RevAnimation::new(
                    textures,
                    durations,
            //        canvas.texture().clone(),
           //         frames,
         //           durations[0],
                ).expect("Failed to create Anim");
                animation.current_mut().set_repeating(true);
                animations.lock().unwrap().insert(file_stem, animation);
         //   } else {
         //       logger.log_error("AssetLoader", "Unable to create new Drawing Canvas, OpenGL error.");
         //   }
        };

        logger.log("AssetLoader", "Loaded Animations");
        Ok(AssetLoader {
            animations: Arc::try_unwrap(animations).unwrap().into_inner().unwrap(),
            textures: HashMap::new(),
            fonts: HashMap::new(),
        })
    }
}
