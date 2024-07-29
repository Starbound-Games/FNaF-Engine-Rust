use tetra::window;
use delta;
pub struct OfficeRenderer;

impl OfficeRenderer {
    pub fn render(ctx: &mut Context, engine: &mut EngineData, cache: &mut CacheData, delta_time: Duration) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.0, 0.0, 0.0));

        if let Some(curstate) = cache.textures.get(&engine.officemgr.game_data.office.states[&engine.officemgr.game_data.office.state]) {
            curstate.draw(ctx, Vec2::new(-engine.officemgr.scroll_x, 0.0));
        }

        for object in &engine.game.offices[&engine.officemgr.curoffice].objects {
            let pos_offset: Vec2<f32> =
                Vec2::new(object.position[0] as f32 * 2.13 - engine.officemgr.scroll_x, object.position[1] as f32 * 2.13 + 1.0);

            match object.r#type.as_str() {
                "door_button" if engine.officemgr.game_data.office.objects[&object.id].visible && !engine.needs_recache => {
                    let mut door_vars = engine.officemgr.game_data.office.doors.get_mut(&object.id).expect("FAILED TO GET DOOR");
                    let button = engine.buttons.get(&object.id).unwrap();
                    if button.is_hovered {
                        if button.is_clicked {
                            if door_vars.button.clickable {
                                if door_vars.is_closed {
                                    door_vars.button.is_on = false;
                                    door_vars.is_closed = false;
                                } else {
                                    door_vars.button.is_on = true;
                                    door_vars.is_closed = true;
                                }
                                door_vars.button.clickable = false;
                            }
                        } else {
                            door_vars.button.clickable = true;
                        }
                    }
                    let texture_key = if door_vars.button.is_on {
                        &object.on_sprite
                    } else {
                        &object.sprite
                    };
                    cache.textures[texture_key].draw(ctx, pos_offset);
                }
                "light_button" => if engine.officemgr.game_data.office.objects[&object.id].visible {
                    if engine.officemgr.game_data.office.lights[&object.id].is_on {
                        cache.textures[&object.on_sprite].draw(ctx, pos_offset);
                    } else {
                        cache.textures[&object.sprite].draw(ctx, pos_offset);
                    }
                }
                "sprite" if engine.officemgr.game_data.office.objects[&object.id].visible && !object.sprite.is_empty() => {
                    cache.textures[&object.sprite].draw(ctx, pos_offset);
                }
                "animation" => if engine.officemgr.game_data.office.objects[&object.id].visible &&
                                engine.officemgr.game_data.office.animations.get(&object.id).expect("FAILED TO GET ANIMATION").is_playing {
                    let anim = cache.animations.get_mut(&object.animation).expect("FAILED TO GET ANIMATION");
                    anim.advance(ctx);
                    anim.draw(ctx, pos_offset);
                }
                "door" => if engine.officemgr.game_data.office.objects[&object.id].visible {
                    let mut anim = cache.animations.get_mut(&object.animation).expect("FAILED TO GET ANIMATION");
                    if (anim.current().repeating())
                    {
                        anim.current_mut().set_repeating(false);
                    }
                    if (!engine.officemgr.game_data.office.doors.get_mut(&object.id).unwrap().is_closed)
                    {
                        anim.set_state(AnimationState::Reverse)
                    }
                    else {
                        anim.set_state(AnimationState::Normal)
                    }
                    anim.advance(ctx);
                    anim.draw(ctx, pos_offset);
                }
                _ => {}
            }
        }


     //   if &engine.officemgr.Office.



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
