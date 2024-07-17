use tetra::graphics::{self, Color, Rectangle, Texture};
use tetra::graphics::text::{Font, Text};
use tetra::math::num_traits::ToPrimitive;
use tetra::math::Vec2;
use std::option::Option;

pub struct MenuRenderer;

impl MenuRenderer {
    pub fn render(ctx: &mut Context, engine: &mut EngineData, cache: &mut CacheData) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.0, 0.0, 0.0));

        if let Some(background) = cache.textures.get(&engine.game.menus[&engine.menumgr.curmenu].properties.BackgroundImage) {
            background.draw(
                ctx,
                Vec2::zero(),
            );
        }
        for element in &engine.game.menus[&engine.menumgr.curmenu].elements {
            let mut pos_offset: Vec2<f32> =
                Vec2::new(element.x as f32 * 2.15, element.y as f32 * 2.15);

            match element.r#type.as_str() {
                ("Button") if !element.hidden => {
                    let uid = format!("{}-{}", &element.text, &element.fontsize);
                    let mut text = cache.texts
                        .entry(uid.clone())
                        .or_insert_with(|| Text::new(&element.text, cache.fonts[&element.fontsize].clone()));

                    let button = &engine.buttons[&element.id.to_string()];
                    if button.is_hovered && !text.content().starts_with(">>") {
                        text.set_content(format!(">> {}", &element.text));
                    } else if !button.is_hovered && text.content().starts_with(">>") {
                        text.set_content(&element.text);
                    }

                    pos_offset.x -= text.get_bounds(ctx).unwrap().width - button.bounds.width - 5.0;
                    text.draw(ctx, pos_offset);
                }
                ("StaticText") if !element.hidden => {
                    let uid = format!("{}-{}", &element.text, &element.fontsize);

                    cache.texts
                        .entry(uid.clone())
                        .or_insert_with(|| Text::new(&element.text, cache.fonts[&element.fontsize].clone()))
                        .draw(ctx, pos_offset);
                }
                "Image" if !element.hidden && !element.sprite.is_empty() => {
                    cache.textures[&element.sprite]
                        .draw(ctx, pos_offset);
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


