use notan::draw::{CreateDraw, DrawShapes};
use notan::prelude::{Color, Graphics};

pub struct Game {}

impl Game {
    pub fn draw(gfx: &mut Graphics) {
        let mut draw = gfx.create_draw();
        draw.clear(Color::TEAL);

        draw.rect((100.0, 100.0), (600.0, 400.0));

        gfx.render(&draw);
    }
}