extern crate font_loader as fonts;
use tetra::graphics::text;
use std::error::Error;
use fonts::system_fonts;

struct FontLoader
{

}
fn path_from_name(font_name: &str) -> String
{
    let sysfonts = system_fonts::query_all();
    if sysfonts.contains(&font_name.to_string()) {
        return sysfonts.iter().find(|&r| r == font_name).unwrap().clone()
    }
    else
    {
        panic!("sussy baka!!! where the hell is the font youre trying to get!??")
    }
}

impl FontLoader
{
    fn font_from_name(ctx: &mut Context, font_name: &str, font_size: i32) -> tetra::Result<Font>
    {
        Ok(Font::vector(ctx, path_from_name(font_name), font_size as f32)?)
    }
}