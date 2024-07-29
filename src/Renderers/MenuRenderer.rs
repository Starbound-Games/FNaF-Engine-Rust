use tetra::graphics::{self, Color, Rectangle, Texture};
use tetra::graphics::text::{Font, Text};
use tetra::math::num_traits::ToPrimitive;
use tetra::math::Vec2;
use std::option::Option;

pub struct MenuRenderer;

impl MenuRenderer {
    pub fn render(ctx: &mut Context, engine: &mut EngineData, cache: &mut CacheData) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.0, 0.0, 0.0));
        let BG_Path = &engine.game.menus[&engine.menumgr.curmenu].properties.BackgroundImage;
        if (!BG_Path.is_empty()) {
            cache.textures
                .entry(BG_Path.clone())
                .or_insert_with(||
                    Texture::new(ctx, engine.assets
                        .join("sprites")
                        .join(&BG_Path.replace("\\", "/"))
                        .to_str()
                        .unwrap()
                        .to_string()).unwrap())
                .draw(ctx, Vec2::zero());
        }
        for element in &engine.game.menus[&engine.menumgr.curmenu].elements {
            let mut pos_offset: Vec2<f32> =
                Vec2::new(element.x as f32 * 2.15, element.y as f32 * 2.15);

            match element.r#type.as_str() {
                ("Button") if !element.hidden => {
                    let uid = format!("{}-{}", &element.text, &element.fontsize);

                    cache.texts
                        .entry(uid.clone())
                        .or_insert_with(|| Text::new(&element.text, cache.fonts[&element.fontsize].clone())).draw(ctx, pos_offset);

                    if  engine.menumgr.arrows_enabled && engine.buttons[&element.id].is_hovered
                    {
                        let suid = format!("{}-{}", ">> ", &element.fontsize);
                        let mut arr_offset = pos_offset.clone();
                        arr_offset.x -= (&element.fontsize * 2) as f32;
                        cache.texts
                            .entry(suid)
                            .or_insert_with(|| Text::new(">> ", cache.fonts[&element.fontsize].clone())).draw(ctx, arr_offset);
                    }
                }
                ("StaticText") if !element.hidden => {
                    let uid = format!("{}-{}", &element.text, &element.fontsize);

                    cache.texts
                        .entry(uid.clone())
                        .or_insert_with(|| Text::new(&element.text, cache.fonts[&element.fontsize].clone()))
                        .draw(ctx, pos_offset);
                }
                "Image" if !element.hidden && !element.sprite.is_empty() => {
                    cache.textures
                        .entry(element.sprite.clone())
                        .or_insert_with( ||
                            Texture::new(ctx, engine.assets
                                .join("sprites")
                                .join(&element.sprite.replace("\\", "/"))
                                .to_str()
                                .unwrap()
                                .to_string()).unwrap())
                        .draw(ctx, pos_offset);
                }
                "Animation" => {
                    let mut anim = cache.animations.get_mut(&element.animation.clone()).expect("FAILED TO GET ANIMATION");
                    anim.advance(ctx);
                    anim.draw(ctx, pos_offset);
                }
                _ => {}
            }
        }
        engine.fps.update(engine.stopwatch.elapsed().as_secs_f64());
        Text::new(
            engine.fps.framerate().to_i32().unwrap().to_string(),
            Font::vector(ctx, "./src/Arial.ttf", 24.0)?,
        )
            .draw(ctx, Vec2::new(16.0, 16.0));
        engine.stopwatch = std::time::Instant::now();

        Ok(())
    }
}


