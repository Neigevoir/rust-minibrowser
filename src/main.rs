mod render;
mod layout;
mod dom;
mod style;

use dom::loadDoc;
use render::{drawRenderBox, draw_rect, Point, Size};

use minifb::{ Window, WindowOptions,};
use raqote::{DrawTarget, SolidSource, Source};
use font_kit::family_name::FamilyName;
use font_kit::properties::Properties;
use font_kit::source::SystemSource;


const WIDTH: usize = 400;
const HEIGHT: usize = 400;

fn main() {
    let styles = style::make_examples();


    let mut window = Window::new("Raqote", WIDTH, HEIGHT, WindowOptions {
                                    ..WindowOptions::default()
                                }).unwrap();
    let font = SystemSource::new()
        .select_best_match(&[FamilyName::SansSerif], &Properties::new())
        .unwrap()
        .load()
        .unwrap();

    let size = window.get_size();
    

    let doc = load_doc("test1.json");
    let bbox = layout::perform_layout(&doc, &styles, &font, (size.0 - 100) as i32);
    let red:Source = Source::Solid(SolidSource::from_unpremultiplied_argb(0xff, 0xff, 0x00, 0));


    let mut dt = DrawTarget::new(size.0 as i32, size.1 as i32);
    loop {
        dt.clear(SolidSource::from_unpremultiplied_argb(0xff, 0xff, 0xff, 0xff));
        drawRenderBox(&mut dt, &bbox, &font);
        draw_rect(&mut dt, &Point{x:width, y:0}, &Size{w:1, h:size.1 as i32}, &RED);
        window.update_with_buffer(dt.get_data(), size.0, size.1).unwrap();
    }
}

