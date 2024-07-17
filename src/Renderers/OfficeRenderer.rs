
pub struct OfficeRenderer;

impl OfficeRenderer {

    pub fn render(ctx: &mut Context, engine: &mut EngineData, cache: &mut CacheData) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.0, 0.0, 0.0));

        if let Some(curstate) = cache.textures.get(&engine.officemgr.Office.states[&engine.officemgr.Office.state]) {
            curstate.draw(
                ctx,
                Vec2::zero(),
            );
        }
        for object in &engine.game.offices[&engine.officemgr.curoffice].objects {
            let pos_offset: Vec2<f32> =
                Vec2::new(object.position[0] as f32 * 2.13, object.position[1] as f32 * 2.13);

            match object.r#type.as_str() {
                "door_button" if engine.officemgr.Office.sprites[&object.id].visible => {
                    if engine.officemgr.Office.doors[&object.id].button.is_on
                    {
                        cache.textures[&object.on_sprite]
                            .draw(ctx, pos_offset);
                    }
                    else
                    {
                        cache.textures[&object.sprite]
                            .draw(ctx, pos_offset);
                    }
                }
                "light_button" if engine.officemgr.Office.sprites[&object.id].visible => {
                //    if engine.officemgr.Office.lights[&object.id].is_on
                //    {
                //        cache.textures[&object.on_sprite]
                //            .draw(ctx, pos_offset);
                //    }
                //    else
                //    {
                //        cache.textures[&object.sprite]
                //            .draw(ctx, pos_offset);
                //}
                }
                "sprite" if engine.officemgr.Office.sprites[&object.id].visible && !object.sprite.is_empty() => {
                    cache.textures[&object.sprite]
                        .draw(ctx, pos_offset);
                }
                //"animation" if engine.officemgr.Office.sprites[&object.id].visible && !object.sprite.is_empty() => {
                //}
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
