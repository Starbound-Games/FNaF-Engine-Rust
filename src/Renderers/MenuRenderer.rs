use tetra::graphics::{self, Color, Texture};
use tetra::graphics::text::{Font, Text};
use tetra::math::num_traits::ToPrimitive;
use tetra::math::Vec2;

pub struct MenuRenderer;

impl MenuRenderer {
    pub fn render(ctx: &mut Context, state: &mut GameState) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.0, 0.0, 0.0));

        if state.textures.contains_key(&state.game.menus["Main"].properties.BackgroundImage) {
            state.textures[&state.game.menus["Main"].properties.BackgroundImage].draw(
                ctx,
                Vec2::zero(),
            );
        }
        for element in &state.game.menus["Main"].elements {
            let pos_offset: Vec2<f32> =
                Vec2::new(element.x as f32 * 2.15, element.y as f32 * 2.15);

            match element.r#type.as_str() {
                ("Button" | "StaticText") if !element.hidden => {
                    let uid = format!("{}-{}", &element.text, &element.fontsize);

                    state.texts
                        .entry(uid.clone())
                        .or_insert_with(|| Text::new(&element.text, state.fonts[&element.fontsize].clone()))
                        .draw(ctx, pos_offset);
                }
                "Image" if !element.hidden && !element.sprite.is_empty() => {
                    state.textures[&element.sprite]
                        .draw(ctx, pos_offset);
                }
                _ => {}
            }
        }
        state.fps.update(state.stopwatch.elapsed().as_secs_f64());
        Text::new(
            state.fps.framerate().to_i32().unwrap().to_string(),
            Font::vector(ctx, "./src/Arial.ttf", 24.0)?,
        )
            .draw(ctx, Vec2::new(16.0, 16.0));
        state.stopwatch = std::time::Instant::now();

        Ok(())
    }
}
