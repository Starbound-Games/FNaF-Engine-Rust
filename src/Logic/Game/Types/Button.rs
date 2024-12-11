use std::ffi::CString;
use tetra::input::{self, MouseButton};

#[derive(Clone)]
pub struct Button2D {
    pub bounds: Rectangle,
    pub event: String,
    pub event_args: Vec<String>,
    pub is_hovered: bool,
    pub is_clicked: bool,
    pub is_clickable: bool,
    pub is_image: bool,
    x: f32
}

impl Button2D {
    pub fn new(ctx: &mut Context, position: Vec2<f32>, texture: Option<&Texture>, mut text: Option<&mut Text>, event: String, event_args: Vec<String>) -> Self {
        let x_scale: f32 = (window::get_width(ctx) / 1280) as f32;
        let y_scale: f32 = (window::get_height(ctx)  / 720) as f32;
        let mut bounds: Rectangle = match &texture {
            Some(tex) => Rectangle::new(-5.0, -5.0, tex.width() as f32 + 5.0, tex.height() as f32 + 5.0),
            None => {
                let mut bound = text.expect("FAILED TO GET BOUNDS").get_bounds(ctx).unwrap();
                bound.x -= 5.0;
                bound.y -= 5.0;
                bound.height += 5.0;
                bound.width += 5.0;

                bound
            },
        };

        bounds.x = position[0] - 5.0;
        bounds.y = position[1] - 5.0;
        bounds.height *= x_scale;
        bounds.width *= y_scale;

        Self {
            bounds,
            event,
            event_args,
            is_hovered: false,
            is_clicked: false,
            is_clickable: true,
            is_image: texture.is_some(),
            x: bounds.x
        }
    }

    pub fn update(&mut self, ctx: &mut Context, x_offset: f32) {
        self.bounds.x = self.x - x_offset;
        self.is_hovered = self.bounds.contains_point(input::get_mouse_position(ctx));

        self.is_clicked = self.is_clickable && input::is_mouse_button_down(ctx, MouseButton::Left) &&
            self.is_hovered;

        self.is_clickable = input::is_mouse_button_up(ctx, MouseButton::Left) &&
            !self.is_clicked;
    }
}
